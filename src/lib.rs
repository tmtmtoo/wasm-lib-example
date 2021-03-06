extern crate octavo_digest as digest;
extern crate octavo_mac as mac;
extern crate wasm_bindgen;

use mac::hmac::Hmac;
use mac::Mac;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hmac_with_secret(message: &str) -> Vec<u8> {
    let secret = b"some_secret";

    let mut hmac_sha256 = Hmac::<digest::sha2::Sha256>::new(secret);
    hmac_sha256.update(message);

    let mut output = [0; 32];
    hmac_sha256.result(&mut output);

    output.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let except = vec![
            157, 92, 228, 175, 135, 190, 204, 185, 189, 175, 179, 183, 247, 34, 162, 147, 44, 196,
            233, 30, 168, 152, 53, 149, 36, 74, 121, 240, 126, 107, 255, 201,
        ];
        let actual = hmac_with_secret("some_message");

        assert_eq!(actual, except);
    }
}
