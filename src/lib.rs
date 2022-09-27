use std::io::Read;
use std::{fmt::Write, num::ParseIntError};

use binary_utils::{convert_to_binary_string, get_binary_string_for_byte_array};

// Should work like this: http://www.unit-conversion.info/texttools/hexadecimal/
pub struct Error;

pub fn convert_decimal_to_32_byte_hex(num: u32) -> String {
    format!("{:08x}", num)
}
pub fn convert_decimal_to_8_byte_hex(num: u8) -> String {
    format!("{:02x}", num)
}
pub fn convert_hex_to_decimal(hex: &str) -> Result<i64, ParseIntError> {
    let z = i64::from_str_radix(hex, 16);
    z
}

pub fn get_text_for_hex(hex: &String) -> Result<String, Error> {
    let maybe_hex_utf8 = convert_hex_utf8(&hex);
    match maybe_hex_utf8 {
        Ok(hex_utf8) => Ok(hex_utf8),
        Err(_error) => {
            let maybe_hex_ascii = convert_hex_to_ascii(&hex);
            match maybe_hex_ascii {
                Ok(hex_ascii) => Ok(hex_ascii),
                Err(_error) => Err(Error),
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
                Err(_error) => Err(Error),
            }
        }
        Err(_error) => Err(Error),
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
        Err(_error) => Err(Error),
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let len = s.len();
    let is_len_odd = len % 2 == 1;
    let s_twos_complement = if is_len_odd {
        format!("0{}", s)
    } else {
        s.to_string()
    };
    (0..s_twos_complement.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s_twos_complement[i..i + 2], 16))
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
            5 => format!("{:010x}", decimal_num),
            6 => format!("{:012x}", decimal_num),
            7 => format!("{:014x}", decimal_num),
            8 => format!("{:016x}", decimal_num),
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
pub fn binary_to_hex(b: &str) -> Option<&str> {
    match b {
        "0000" => Some("0"),
        "0001" => Some("1"),
        "0010" => Some("2"),
        "0011" => Some("3"),
        "0100" => Some("4"),
        "0101" => Some("5"),
        "0110" => Some("6"),
        "0111" => Some("7"),
        "1000" => Some("8"),
        "1001" => Some("9"),
        "1010" => Some("A"),
        "1011" => Some("B"),
        "1100" => Some("C"),
        "1101" => Some("D"),
        "1110" => Some("E"),
        "1111" => Some("F"),
        _ => None,
    }
}
pub fn convert_string_to_hex(s: &String) -> String {
    let wif_bytes = s.as_bytes();
    let binary = get_binary_string_for_byte_array(&wif_bytes.to_vec());

    let mut s = String::new();
    let mut b = String::new();
    for byte in wif_bytes {
        let binary_string = convert_to_binary_string(*byte, 8);

        let first_4_binary = &binary_string[0..=3];
        let first_4_hex = binary_to_hex(first_4_binary).unwrap();
        let last_4_binary = &binary_string[4..=7];
        let last_4_hex = binary_to_hex(last_4_binary).unwrap();
        let to_p = format!("{}{}", first_4_hex, last_4_hex);

        s.push_str(&to_p);
    }
    s
}

pub fn get_hex_string_from_byte_array(byte_array: &[u8]) -> String {
    // Use that array to then create a length 32 array but with hexidecimal values, since we want
    // each item of the array to represent only 4 bits, which is how many bits a hex represents
    let array_with_base_16_numbers: Vec<u8> = byte_array.iter().map(|num| num % 16).collect();
    // turn hex byte array into hex string
    let hex_string = array_with_base_16_numbers
        .iter()
        .map(|byte| format!("{:x}", byte))
        .collect::<String>();
    hex_string
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write tests
    #[test]
    fn it_works() {
        // test: http://www.unit-conversion.info/texttools/hexadecimal/
        let hex = "30784e6f6e63652077617320666f756e646564".to_string();
        let expected_text = "0xNonce was founded".to_string();
        let maybe_text = get_text_for_hex(&hex);
        let text = match maybe_text {
            Ok(text) => text.to_string(),
            Err(_) => "wrong".to_string(),
        };
        assert_eq!(text, expected_text);
    }
    #[test]
    fn decode_hex_test() {
        let hex = "d473a59";
        let decoded = decode_hex(hex);
    }
}
