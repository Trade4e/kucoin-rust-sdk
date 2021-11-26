use std::io::Write;
use reqwest::{Error, Response};
use crate::client::{BaseUrl, Client, Environment};
use serde::{Deserialize, Serialize};

type Code = String;
type Msg = String;
type TimeStampData = i64;

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct TimeStamp {
    code:Code,
    msg:Option<Msg>,
    data:Option<TimeStampData>
}

impl TimeStamp {
    async fn get(env:Environment) -> TimeStamp {
        let base_url = BaseUrl::create_base_url(env).value();
        let url = format!("{}/api/v1/timestamp", base_url);
        reqwest::Client::new().get(url).send().await.unwrap().json().await.unwrap()
    }
}

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct Status {
    code: Code,
    data: StatusData
}

impl Status {
    async fn get(env:Environment) -> Status {
        let base_url = BaseUrl::create_base_url(env).value();
        let url = format!("{}/api/v1/status", base_url);
        reqwest::Client::new().get(url).send().await.unwrap().json().await.unwrap()
    }
}

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct StatusData {
    status: StatusOfService, //open, close, cancelonly
    msg:  Msg   //remark for operation
}

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
enum StatusOfService {
    open,
    close,
    cancelonly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timestamp_get() {
        let timestamp = TimeStamp::get(Environment::Sandbox).await;
        println!("{:#?}", timestamp)
    }

    #[tokio::test]
    async fn test_status_get() {
        let status = Status::get(Environment::Sandbox).await;
        println!("{:#?}", status)
    }
}