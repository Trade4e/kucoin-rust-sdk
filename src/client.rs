use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Method, RequestBuilder, Request};
use crate::utils::get_time_as_millis;

type Key = String;
type Secret = String;
type Passphrase = String;

#[derive(Debug , PartialOrd, PartialEq)]
pub struct BaseUrl(String);
impl BaseUrl {
    pub fn create_base_url(env:Environment) -> BaseUrl {
        let url = match env {
            Environment::Live => {"https://api.kucoin.com".to_string()},
            Environment::Sandbox => {"https://openapi-sandbox.kucoin.com".to_string()}
        };
        BaseUrl(url)
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}


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
        Client{
            key: key.to_string(),
            secret: secret.to_string(),
            passphrase: passphrase.to_string(),
            base_url: BaseUrl::create_base_url(env)
        }
    }

    pub fn get(&self, uri:&str) -> RequestBuilder {
        create_request_without_header(Method::GET, uri)
    }

    pub fn post(&self) -> Client {
        unimplemented!()
    }

    pub fn put(&self) -> Client {
        unimplemented!()
    }

    pub fn delete(&self) -> Client {
        unimplemented!()
    }
}


fn create_request(method:Method, uri:&str, header:HeaderMap) -> reqwest::Result<Request> {
    reqwest::Client::new().request(method, uri).headers(header).build()
}

fn create_request_without_header(method:Method, uri:&str) -> RequestBuilder {
    reqwest::Client::new().request(method, uri)
}

fn header_builder(key:&str, timestamp:&str, passphrase:&str, signature:&str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
    headers.insert(HeaderName::from_static("kc-api-key-version"), HeaderValue::from(2));
    headers.insert(HeaderName::from_static("kc-api-timestamp"), HeaderValue::from_str(timestamp).unwrap());
    headers.insert(HeaderName::from_static("kc-api-key"), HeaderValue::from_str(key).unwrap());
    headers.insert(HeaderName::from_static("kc-api-sign"), HeaderValue::from_str(signature).unwrap());
    headers.insert(HeaderName::from_static("kc-api-passphrase"), HeaderValue::from_str(passphrase).unwrap());
    headers
}

fn create_str_to_sign_with_parameter(timestamp:&str, method:&str, param:&str, request_path:&str) -> String {
    format!("{}{}{}{}", timestamp, method, request_path, param)
}

fn create_str_to_sign_without_parameter(timestamp:&str, method:&str, request_path:&str) -> String {
    format!("{}{}{}", timestamp, method, request_path)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_request() {
        let h = header_builder("a", "b", "c", "d");
        let r = create_request(Method::GET, "https://google.com", h);
        println!("{:#?}", r)
    }

    #[test]
    fn test_client_create() {
        let live_client = Client {
            key: "key_test".to_string(),
            secret: "secret_test".to_string(),
            passphrase: "passphrase_test".to_string(),
            base_url: BaseUrl::create_base_url(Environment::Live)
        };

        let sandbox_client = Client {
            key: "key_test".to_string(),
            secret: "secret_test".to_string(),
            passphrase: "passphrase_test".to_string(),
            base_url: BaseUrl::create_base_url(Environment::Sandbox)
        };
        let new_live_client = Client::create("key_test","secret_test","passphrase_test",  Environment::Live);
        let new_sandbox_client = Client::create("key_test","secret_test","passphrase_test",  Environment::Sandbox);
        assert_eq!(live_client, new_live_client);
        assert_eq!(sandbox_client, new_sandbox_client);
    }
}