use hex;
use base64;

pub fn decode_hex_string(input: String) -> String {
    let decode_result = hex::decode(input);
    let str_vector = decode_result.unwrap();
    String::from_utf8(str_vector).unwrap()
}

pub fn encode_base64_string(input: String) -> String {
    base64::encode(input)
}

