//! Typed browser audio dictation state machine and `ui::Dictation` wiring.

const MAX_WAVE_BARS: usize = 40;

/// Lifecycle state of voice dictation inside the active composer.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum DictationState {
    /// No active dictation session.
    #[default]
    Idle,
    /// Microphone capture is live and streaming loudness samples.
    Listening {
        /// Epoch milliseconds when recording started.
        started_ms: f64,
        /// Elapsed whole seconds recorded so far.
        elapsed_seconds: u32,
        /// Recent normalized RMS waveform samples (`0.0..=1.0`).
        levels: Vec<f32>,
    },
    /// Audio finished recording and `POST /dictation` transcription is in flight.
    Transcribing {
        /// Frozen formatted duration (`M:SS`).
        elapsed: String,
        /// Frozen waveform samples.
        levels: Vec<f32>,
    },
}

impl DictationState {
    pub(crate) fn is_active(&self) -> bool {
        !matches!(self, Self::Idle)
    }

    pub(crate) fn is_listening(&self) -> bool {
        matches!(self, Self::Listening { .. })
    }

    pub(crate) fn start(started_ms: f64) -> Self {
        Self::Listening {
            started_ms,
            elapsed_seconds: 0,
            levels: Vec::new(),
        }
    }

    pub(crate) fn push_level(&mut self, level: f32, now_ms: f64) {
        let Self::Listening {
            started_ms,
            elapsed_seconds,
            levels,
        } = self
        else {
            return;
        };
        levels.push(level.clamp(0.0, 1.0));
        if levels.len() > MAX_WAVE_BARS {
            let overflow = levels.len() - MAX_WAVE_BARS;
            levels.drain(..overflow);
        }
        if now_ms >= *started_ms {
            *elapsed_seconds = ((now_ms - *started_ms) / 1000.0).floor() as u32;
        }
    }

    pub(crate) fn into_transcribing(self) -> Self {
        match self {
            Self::Listening {
                elapsed_seconds,
                levels,
                ..
            } => Self::Transcribing {
                elapsed: format_elapsed(elapsed_seconds),
                levels,
            },
            Self::Transcribing { elapsed, levels } => Self::Transcribing { elapsed, levels },
            Self::Idle => Self::Transcribing {
                elapsed: format_elapsed(0),
                levels: Vec::new(),
            },
        }
    }
}

/// Formats elapsed recording seconds cleanly as `M:SS`.
pub(crate) fn format_elapsed(seconds: u32) -> String {
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transitions_cleanly_through_listening_levels_and_transcribing() {
        let mut state = DictationState::start(1_000.0);
        assert!(state.is_listening());
        for idx in 0..45 {
            state.push_level(0.5, 1_000.0 + f64::from(idx) * 100.0);
        }
        let DictationState::Listening {
            elapsed_seconds,
            levels,
            ..
        } = &state
        else {
            panic!("expected Listening");
        };
        assert_eq!(levels.len(), MAX_WAVE_BARS);
        assert_eq!(*elapsed_seconds, 4);

        let transcribing = state.into_transcribing();
        assert_eq!(
            transcribing,
            DictationState::Transcribing {
                elapsed: "0:04".to_owned(),
                levels: vec![0.5; MAX_WAVE_BARS],
            }
        );
    }
}
