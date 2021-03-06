use std::convert::TryInto;

use super::errors::IdentifierError;
use crate::util::string_encoding::Encoding;
use crate::util::string_encoding;

pub struct Identifier {
    buffer: [u8; 32]
}

fn encoding_string_to_encoding(encoding_string: Option<&str>) -> Encoding {
    match encoding_string {
        Some(str) => {
            if str == "base58" {
                Encoding::Base58
            } else {
                Encoding::Base64
            }
        },
        None => Encoding::Base58
    }
}

impl Identifier {
    fn new(buffer: [u8; 32]) -> Identifier {
        Identifier {
            buffer
        }
    }

    pub fn from_string(encoded_value: &str, encoding: Encoding) -> Result<Identifier, IdentifierError> {
        let vec = string_encoding::decode(encoded_value, encoding)?;

        Identifier::from_bytes(&vec)
    }

    pub fn from_string_with_encoding_string(encoded_value: &str, encoding_string: Option<&str>) -> Result<Identifier, IdentifierError> {
        let encoding = encoding_string_to_encoding(encoding_string);

        Identifier::from_string(encoded_value, encoding)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Identifier, IdentifierError> {
        if bytes.len() != 32 {
            return Err(IdentifierError { message: "Identifier must be 32 bytes long".to_string() });
        }

        // Since we checked that vector size is 32, we can use unwrap
        Ok(Identifier::new(bytes.try_into().unwrap()))
    }

    pub fn to_buffer(&self) -> [u8; 32] {
        self.buffer
    }

    pub fn to_string(&self, encoding: Encoding) -> String {
        string_encoding::encode(&self.buffer, encoding)
    }

    pub fn to_string_with_encoding_string(&self, encoding_string: Option<&str>) -> String {
        let encoding = encoding_string_to_encoding(encoding_string);

        self.to_string(encoding)
    }
}