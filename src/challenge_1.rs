use cryptopals::utils;

pub fn challenge_1() -> bool {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_str = hex_to_base64(String::from(hex_str));
    
    let success = base64_str == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        success
}

fn hex_to_base64(input: String) -> String {
    let reg_str = utils::decode_hex_string(input);
    let base64_str = utils::encode_base64_string(reg_str);

    String::from(base64_str)
}
