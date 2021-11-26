use std::time::{SystemTime, UNIX_EPOCH};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

type HmacSha256 = Hmac<Sha256>;

pub fn get_time_as_millis() -> u128 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    since_the_epoch.as_millis()
}

fn sign_to_str(key:&str, str_to_sign:&str) -> String{
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(str_to_sign.as_bytes());

    let result = mac.finalize();
    let bites = result.into_bytes();
    base64::encode(bites)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_to_str() {
        let crypto = sign_to_str("5c2db93503aa674c74a31734", "stringtosign");
        assert_eq!(crypto, "th+wiJekxqm1pvzSZg80V/fZpZz/hyqYJKImxTR+hwE=")
    }
}

