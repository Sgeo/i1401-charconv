use std::{collections::HashMap, sync::LazyLock};

use thiserror::Error;
use serde::Deserialize;

static ENCODINGS_STR: &'static str = include_str!("../encodings.json");

pub trait Encoding {
    fn encode(&self, bcd: u8) -> Result<char, EncodingError>;
    fn decode(&self, char: char) -> Result<u8, EncodingError>;
}

#[derive(Debug, Deserialize)]
struct JsonEncoding {
    encode: Vec<char>,
    decode: HashMap<char, u8>
}

impl Encoding for JsonEncoding {
    fn encode(&self, bcd: u8) -> Result<char, EncodingError> {
        self.encode.get(bcd as usize).copied().ok_or(EncodingError::NoEncode(bcd))
    }
    fn decode(&self, char: char) -> Result<u8, EncodingError> {
        self.decode.get(&char).copied().ok_or(EncodingError::NoDecode(char))
    }
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Cannot convert from character to BCD")]
    NoDecode(char),
    #[error("Cannot convert from BCD to character")]
    NoEncode(u8)
}

#[derive(Debug, Deserialize)]
struct JsonEncodings {
    #[serde(flatten)]
    encodings: HashMap<String, JsonEncoding>
}

pub fn get_encoding(encoding_name: &str) -> Option<&'static dyn Encoding> {
    static ENCODINGS: LazyLock<JsonEncodings> = LazyLock::new(|| serde_json::from_str(ENCODINGS_STR).expect("Unable to deserialize built-in encodings.json!"));
    ENCODINGS.encodings.get(encoding_name).map(|encoding| encoding as &dyn Encoding)
}

