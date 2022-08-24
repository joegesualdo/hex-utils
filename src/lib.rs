use std::{env, fmt::LowerHex, io::Read};
use std::{fmt::Write, num::ParseIntError};

// Should work like this: http://www.unit-conversion.info/texttools/hexadecimal/
pub struct Error;

pub fn get_text_for_hex(hex: &String) -> Result<String, Error> {
    let maybe_hex_utf8 = convert_hex_utf8(&hex);
    match maybe_hex_utf8 {
        Ok(hex_utf8) => Ok(hex_utf8),
        Err(error) => {
            let maybe_hex_ascii = convert_hex_to_ascii(&hex);
            match maybe_hex_ascii {
                Ok(hex_ascii) => Ok(hex_ascii),
                Err(error) => Err(Error),
            }
        }
    }
}

pub fn convert_hex_utf8(hex: &String) -> Result<String, Error> {
    let maybe_decoded_hex = decode_hex(&hex.to_string());
    match maybe_decoded_hex {
        Ok(decoded_hex) => {
            let maybe_utf_str = std::str::from_utf8(&decoded_hex);
            match maybe_utf_str {
                Ok(utf_str) => Ok(utf_str.to_string()),
                Err(error) => Err(Error),
            }
        }
        Err(error) => Err(Error),
    }
}

pub fn convert_hex_to_ascii(hex: &String) -> Result<String, Error> {
    enum Error {
        Int(ParseIntError),
        Unicode(u32),
    }
    fn hex_to_char(s: &str) -> Result<char, Error> {
        // u8::from_str_radix(s, 16).map(|n| n as char)
        // u8::from_str_radix(s, 16).map(|n| n as char)
        let unicode = u32::from_str_radix(s, 16).map_err(Error::Int)?;
        char::from_u32(unicode).ok_or_else(|| Error::Unicode(unicode))
    }

    let maybe_decoded_hex = decode_hex(&hex.to_string());
    match maybe_decoded_hex {
        Ok(decoded_hex) => {
            let mut new_string: String = String::new();
            for maybe_byte in decoded_hex.bytes() {
                match maybe_byte {
                    Ok(byte) => {
                        let hex = format!("{:02X}", byte);
                        let maybe_char = hex_to_char(&hex);
                        match maybe_char {
                            Ok(char) => {
                                new_string.push(char);
                            }
                            Err(_) => return Err(Error),
                        }
                    }
                    Err(_) => return Err(Error),
                }
            }
            Ok(new_string)
        }
        Err(error) => Err(Error),
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        // We want to print the leading zero in each byte array item, so we need 02x formatting
        // here. So "0d" won't be printed as "d"
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn convert_big_endian_hex_to_little_endian(hex: &String) -> String {
    let decoded_hex = decode_hex(&hex).unwrap();
    let reversed_decoded_hex: Vec<u8> = decoded_hex.into_iter().rev().collect();
    let reversed_encoded_hex = encode_hex(&reversed_decoded_hex);
    reversed_encoded_hex
}

pub fn convert_decimal_to_hexadecimal(
    decimal_num: u64,
    include_prefix: bool,
    bytes: Option<u8>,
) -> String {
    let hex_string_without_prefix = match bytes {
        // two characters per byte
        Some(bytes) => match bytes {
            1 => format!("{:02x}", decimal_num),
            2 => format!("{:04x}", decimal_num),
            3 => format!("{:06x}", decimal_num),
            4 => format!("{:08x}", decimal_num),
            _ => panic!("bytes for hex not supported: {}", bytes),
        },
        None => format!("{:x}", decimal_num),
    };
    if include_prefix {
        format!("0x{hex_string_without_prefix}")
    } else {
        hex_string_without_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}