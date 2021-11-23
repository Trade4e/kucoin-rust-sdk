type Key = String;
type Secret = String;
type Passphrase = String;
type BaseUrl= String;

pub enum Environment {
    Live,
    Sandbox
}

#[derive(Debug , PartialOrd, PartialEq)]
pub struct Client {
    key : Key,
    secret: Secret,
    passphrase: Passphrase,
    base_url: BaseUrl
}

impl Client {
    pub fn create(key:&str, secret:&str, passphrase:&str, env:Environment) -> Client {
        let url = match env {
            Environment::Live => {"https://api.kucoin.com".to_string()},
            Environment::Sandbox => {"https://openapi-sandbox.kucoin.com".to_string()}
        };

        Client{
            key: key.to_string(),
            secret: secret.to_string(),
            passphrase: passphrase.to_string(),
            base_url: url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_create() {
        let live_client = Client {
            key: "key_test".to_string(),
            secret: "secret_test".to_string(),
            passphrase: "passphrase_test".to_string(),
            base_url: "https://api.kucoin.com".to_string()
        };

        let sandbox_client = Client {
            key: "key_test".to_string(),
            secret: "secret_test".to_string(),
            passphrase: "passphrase_test".to_string(),
            base_url: "https://openapi-sandbox.kucoin.com".to_string()
        };
        let new_live_client = Client::create("key_test","secret_test","passphrase_test",  Environment::Live);
        let new_sandbox_client = Client::create("key_test","secret_test","passphrase_test",  Environment::Sandbox);
        assert_eq!(live_client, new_live_client);
        assert_eq!(sandbox_client, new_sandbox_client);
    }
}