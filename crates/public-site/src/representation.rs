//! Startup-built HTTP representations; request handling never compresses or hashes.
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, Method, StatusCode, header},
    response::Response,
};
use sha2::{Digest, Sha256};
use std::io::Write;

pub const HTML_CACHE: &str = "public, max-age=300, s-maxage=3600, stale-while-revalidate=86400";
pub const IMMUTABLE_CACHE: &str = "public, max-age=31536000, immutable";

pub fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Clone, Debug)]
struct Variant {
    bytes: Bytes,
    etag: String,
}
impl Variant {
    fn new(bytes: Bytes) -> Self {
        Self {
            etag: format!("\"{}\"", digest(&bytes)),
            bytes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Representation {
    identity: Variant,
    br: Variant,
    gzip: Variant,
    content_type: String,
}

impl Representation {
    pub fn new(bytes: impl Into<Bytes>, content_type: impl Into<String>) -> Self {
        let bytes = bytes.into();
        let mut br = Vec::new();
        {
            let mut writer = brotli::CompressorWriter::new(&mut br, 4096, 6, 22);
            writer.write_all(&bytes).expect("compress in memory");
        }
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(6));
        gz.write_all(&bytes).expect("compress in memory");
        let gzip = gz.finish().expect("finish gzip");
        Self {
            identity: Variant::new(bytes),
            br: Variant::new(br.into()),
            gzip: Variant::new(gzip.into()),
            content_type: content_type.into(),
        }
    }

    pub fn respond(
        &self,
        headers: &HeaderMap,
        method: &Method,
        status: StatusCode,
        cache: &str,
    ) -> Response {
        let Some(encoding) = negotiate(headers) else {
            return Response::builder()
                .status(StatusCode::NOT_ACCEPTABLE)
                .header(header::VARY, "Accept-Encoding")
                .header(header::CACHE_CONTROL, "no-store")
                .body(Body::empty())
                .unwrap();
        };
        let variant = match encoding {
            Encoding::Br => &self.br,
            Encoding::Gzip => &self.gzip,
            Encoding::Identity => &self.identity,
        };
        // If-None-Match uses weak comparison for GET/HEAD, including comma-separated tags.
        let unchanged = status == StatusCode::OK
            && headers
                .get_all(header::IF_NONE_MATCH)
                .iter()
                .filter_map(|value| value.to_str().ok())
                .any(|value| {
                    value.split(',').any(|tag| {
                        let tag = tag.trim().strip_prefix("W/").unwrap_or(tag.trim());
                        tag == "*" || tag == variant.etag
                    })
                });
        let mut builder = Response::builder()
            .status(if unchanged {
                StatusCode::NOT_MODIFIED
            } else {
                status
            })
            .header(header::CONTENT_TYPE, &self.content_type)
            .header(header::CACHE_CONTROL, cache)
            .header(header::VARY, "Accept-Encoding")
            .header(header::ETAG, &variant.etag);
        if let Some(value) = encoding.header() {
            builder = builder.header(header::CONTENT_ENCODING, value);
        }
        if !unchanged {
            builder = builder.header(header::CONTENT_LENGTH, variant.bytes.len());
        }
        builder
            .body(if unchanged || *method == Method::HEAD {
                Body::empty()
            } else {
                Body::from(variant.bytes.clone())
            })
            .unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Encoding {
    Br,
    Gzip,
    Identity,
}
impl Encoding {
    fn header(self) -> Option<&'static str> {
        match self {
            Self::Br => Some("br"),
            Self::Gzip => Some("gzip"),
            Self::Identity => None,
        }
    }
}

// RFC 9110 qvalue grammar, integer arithmetic, no NaN/negative/over-one acceptance.
fn quality(value: &str) -> Option<u16> {
    let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
    if fraction.len() > 3 || !fraction.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    match whole {
        "1" if fraction.bytes().all(|b| b == b'0') => Some(1000),
        "0" => Some(fraction.parse::<u16>().unwrap_or(0) * 10u16.pow(3 - fraction.len() as u32)),
        _ => None,
    }
}

fn negotiate(headers: &HeaderMap) -> Option<Encoding> {
    if !headers.contains_key(header::ACCEPT_ENCODING) {
        return Some(Encoding::Identity);
    }
    let mut values = std::collections::HashMap::<String, u16>::new();
    for header in headers.get_all(header::ACCEPT_ENCODING) {
        let Ok(header) = header.to_str() else {
            return None;
        };
        for item in header.split(',').filter(|item| !item.trim().is_empty()) {
            let mut parts = item.split(';');
            let name = parts.next().unwrap().trim().to_ascii_lowercase();
            let mut q = 1000;
            for parameter in parts {
                match parameter.trim().split_once('=') {
                    Some((key, value)) if key.trim().eq_ignore_ascii_case("q") => {
                        q = quality(value.trim()).unwrap_or(0)
                    }
                    _ => q = 0,
                }
            }
            // Conservative handling of duplicate codings: explicit refusal wins.
            values
                .entry(name)
                .and_modify(|old| *old = (*old).min(q))
                .or_insert(q);
        }
    }
    let wildcard = values.get("*").copied();
    let br = values.get("br").copied().or(wildcard).unwrap_or(0);
    let gzip = values.get("gzip").copied().or(wildcard).unwrap_or(0);
    // Identity is acceptable by default, but should not defeat an explicitly preferred
    // compressed coding solely because its implicit quality is 1.
    let identity = values
        .get("identity")
        .copied()
        .unwrap_or(if wildcard == Some(0) { 0 } else { 1 });
    [
        (Encoding::Br, br),
        (Encoding::Gzip, gzip),
        (Encoding::Identity, identity),
    ]
    .into_iter()
    .filter(|(_, q)| *q > 0)
    .max_by_key(|(encoding, q)| {
        (
            *q,
            match encoding {
                Encoding::Br => 2,
                Encoding::Gzip => 1,
                Encoding::Identity => 0,
            },
        )
    })
    .map(|(encoding, _)| encoding)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pick(value: &str) -> Option<Encoding> {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT_ENCODING, value.parse().unwrap());
        negotiate(&headers)
    }
    #[test]
    fn digest_is_lowercase_sha256() {
        assert_eq!(
            digest(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn gzip_vector_crc_round_trip() {
        // Exercise vectorized CRC folding, which short strings do not reach.
        // This caught a linker alignment fault with inherited ElyLD flags.
        use std::io::Read;
        let input = include_bytes!("../assets/brand/duet/mark.svg");
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(6));
        gz.write_all(input).unwrap();
        let compressed = gz.finish().unwrap();
        let mut decoded = Vec::new();
        flate2::read::GzDecoder::new(&compressed[..])
            .read_to_end(&mut decoded)
            .unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn q_values() {
        assert_eq!(pick("br;q=0.3, gzip;q=0.8"), Some(Encoding::Gzip));
        assert_eq!(pick("br, gzip"), Some(Encoding::Br));
        assert_eq!(pick("br;q=0, gzip;q=0"), Some(Encoding::Identity));
        assert_eq!(pick("*;q=0"), None);
        assert_eq!(pick("*;q=0, gzip;q=0.5"), Some(Encoding::Gzip));
        assert_eq!(pick("br;q=NaN, identity;q=0"), None);
        assert_eq!(pick("br;q=1.1, identity;q=0"), None);
        assert_eq!(pick("gzip;q=0.8, identity;q=1"), Some(Encoding::Identity));
        assert_eq!(pick(""), Some(Encoding::Identity));
    }
    #[tokio::test]
    async fn variants_head_and_weak_conditional() {
        let data = Representation::new("hello hello hello", "text/plain");
        let mut headers = HeaderMap::new();
        let plain = data.respond(&headers, &Method::GET, StatusCode::OK, HTML_CACHE);
        let plain_etag = plain.headers()[header::ETAG].clone();
        headers.insert(header::ACCEPT_ENCODING, "gzip".parse().unwrap());
        let gz = data.respond(&headers, &Method::HEAD, StatusCode::OK, HTML_CACHE);
        assert_ne!(gz.headers()[header::ETAG], plain_etag);
        assert_eq!(gz.headers()[header::CONTENT_ENCODING], "gzip");
        let etag = gz.headers()[header::ETAG].to_str().unwrap().to_string();
        assert!(
            axum::body::to_bytes(gz.into_body(), 1000)
                .await
                .unwrap()
                .is_empty()
        );
        headers.insert(
            header::IF_NONE_MATCH,
            format!("\"other\", W/{etag}").parse().unwrap(),
        );
        let response = data.respond(&headers, &Method::GET, StatusCode::OK, HTML_CACHE);
        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        assert!(response.headers().get(header::CONTENT_LENGTH).is_none());
        assert_eq!(response.headers()[header::VARY], "Accept-Encoding");
        assert!(
            axum::body::to_bytes(response.into_body(), 1000)
                .await
                .unwrap()
                .is_empty()
        );
    }
    #[tokio::test]
    async fn compressed_bytes_round_trip() {
        use std::io::Read;
        let data = Representation::new(
            "testing repeated content testing repeated content",
            "text/plain",
        );
        let mut out = Vec::new();
        brotli::Decompressor::new(&data.br.bytes[..], 4096)
            .read_to_end(&mut out)
            .unwrap();
        assert_eq!(out, data.identity.bytes);
        out.clear();
        flate2::read::GzDecoder::new(&data.gzip.bytes[..])
            .read_to_end(&mut out)
            .unwrap();
        assert_eq!(out, data.identity.bytes);
    }
}
