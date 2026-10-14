use std::path::Path;

use candle_core::{DType, Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, HiddenAct};
use tokenizers::Tokenizer;

use crate::protocol::Encode;
use crate::window::BATCH;

/// The `BertEncoder` value exchanged by this crate.
pub struct BertEncoder {
    model: BertModel,
    tokenizer: Tokenizer,
    cls: u32,
    sep: u32,
    pad: u32,
    device: Device,
}

impl BertEncoder {
    /// Loads the encoder weights from the directory.
    pub fn load(directory: &Path) -> Result<Self, String> {
        let config_path = directory.join("config.json");
        let config_text = std::fs::read_to_string(&config_path)
            .map_err(|error| format!("config.json: {error}"))?;
        let config: Config =
            serde_json::from_str(&config_text).map_err(|error| format!("config.json: {error}"))?;
        if config.model_type.as_deref() != Some("bert") || config.hidden_size != 384 {
            return Err("Unsupported health encoder model".into());
        }
        if config.hidden_act != HiddenAct::Gelu {
            return Err("Unsupported health encoder activation".into());
        }
        let tokenizer = Tokenizer::from_file(directory.join("tokenizer.json"))
            .map_err(|error| format!("tokenizer.json: {error}"))?;
        let cls = token_id(&tokenizer, "[CLS]")?;
        let sep = token_id(&tokenizer, "[SEP]")?;
        let pad = tokenizer.token_to_id("[PAD]").unwrap_or(
            u32::try_from(config.pad_token_id).map_err(|_| "pad token id does not fit u32")?,
        );
        let weights = directory.join("model.safetensors");
        if !weights.is_file() {
            return Err("model.safetensors is missing".into());
        }
        let device = Device::Cpu;
        let builder = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights], DType::F32, &device)
                .map_err(|error| format!("safetensors: {error}"))?
        };
        let model = BertModel::load(builder, &config).map_err(|error| format!("bert: {error}"))?;
        Ok(Self {
            model,
            tokenizer,
            cls,
            sep,
            pad,
            device,
        })
    }

    fn forward_batch(&self, windows: &[Vec<u32>]) -> Result<Vec<Vec<f32>>, String> {
        let wrapped: Vec<Vec<u32>> = windows
            .iter()
            .map(|ids| {
                let mut row = Vec::with_capacity(ids.len() + 2);
                row.push(self.cls);
                row.extend(ids);
                row.push(self.sep);
                row
            })
            .collect();
        let width = wrapped.iter().map(Vec::len).max().unwrap_or(0);
        let mut ids = Vec::with_capacity(wrapped.len() * width);
        let mut mask = Vec::with_capacity(wrapped.len() * width);
        for row in &wrapped {
            ids.extend(row.iter().copied());
            ids.extend(std::iter::repeat_n(self.pad, width - row.len()));
            mask.extend(std::iter::repeat_n(1_u32, row.len()));
            mask.extend(std::iter::repeat_n(0_u32, width - row.len()));
        }
        let rows = wrapped.len();
        let shape = (rows, width);
        let input_ids =
            Tensor::from_vec(ids, shape, &self.device).map_err(|error| error.to_string())?;
        let token_type_ids =
            Tensor::zeros(shape, DType::U32, &self.device).map_err(|error| error.to_string())?;
        let attention_mask =
            Tensor::from_vec(mask, shape, &self.device).map_err(|error| error.to_string())?;
        let hidden = self
            .model
            .forward(&input_ids, &token_type_ids, Some(&attention_mask))
            .map_err(|error| error.to_string())?;
        let cls = hidden.i((.., 0, ..)).map_err(|error| error.to_string())?;
        l2_normalize(&cls)?
            .to_vec2::<f32>()
            .map_err(|error| error.to_string())
    }
}

impl Encode for BertEncoder {
    fn content_ids(&self, text: &str) -> Result<Vec<u32>, String> {
        self.tokenizer
            .encode(text, false)
            .map(|encoding| encoding.get_ids().to_vec())
            .map_err(|error| error.to_string())
    }

    fn forward(&mut self, windows: &[Vec<u32>]) -> Result<Vec<Vec<f32>>, String> {
        let mut vectors = Vec::with_capacity(windows.len());
        for batch in windows.chunks(BATCH) {
            let encoded = self.forward_batch(batch)?;
            if encoded.len() != batch.len() {
                return Err("encoder returned the wrong number of vectors".into());
            }
            vectors.extend(encoded);
        }
        Ok(vectors)
    }
}

fn token_id(tokenizer: &Tokenizer, token: &str) -> Result<u32, String> {
    tokenizer
        .token_to_id(token)
        .ok_or_else(|| format!("tokenizer has no {token} token"))
}

/// L2 along the last dimension, with the same 1e-12 floor as `torch.nn.functional.normalize`.
fn l2_normalize(values: &Tensor) -> Result<Tensor, String> {
    let norm = values
        .sqr()
        .map_err(|error| error.to_string())?
        .sum_keepdim(1)
        .map_err(|error| error.to_string())?
        .sqrt()
        .map_err(|error| error.to_string())?;
    let norm = norm
        .clamp(1e-12, f32::MAX)
        .map_err(|error| error.to_string())?;
    values
        .broadcast_div(&norm)
        .map_err(|error| error.to_string())
}
