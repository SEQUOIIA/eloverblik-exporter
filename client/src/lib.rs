pub mod error;

use reqwest::{Request, Response};
use serde::{Deserialize, Serialize};
use error::{Result, Error};

const BASE_URL : &'static str = "https://api.eloverblik.dk/customerapi";

#[derive(Clone, Debug)]
pub struct Client {
    http : reqwest::Client,
    conf : Config,
    data : ClientData
}

#[derive(Clone, Debug)]
struct ClientData {
    pub token : Option<Token>
}

#[derive(Clone, Debug)]
struct Token {
    pub token : String,
    pub expires_in : i64
}

// Limit most requests to 25 queries every 60 seconds
impl Client {

    // Limit to 2 queries every 60 seconds
    async fn auth(&self) {
        let mut req = Request::new(reqwest::Method::GET, format!("{}/api/token", BASE_URL).parse().unwrap());
        req.headers_mut().insert("Authorization", format!("Bearer {}", self.conf.refresh_token).parse().unwrap());

        let resp = self.check_response(self.http.execute(req).await);
    }

    async fn prepare_http_request(&self, req : &mut Request) {
        if self.data.token.is_none() {
            self.auth().await;
        }

        if self.data.token.is_some() {
            if self.data.token.as_ref().unwrap().expires_in > 1 {
                self.auth().await;
            }
        }

        req.headers_mut().insert("Authorization", format!("Bearer {}", self.data.token.as_ref().unwrap().token).parse().unwrap());
    }

    // Handle rate limits
    fn check_response(&self, resp : std::result::Result<Response, reqwest::Error>) -> Result<Response> {
        match resp {
            Ok(val) => {
                match val.status().as_u16() {
                    429 => Err(Error::ElOverblikRateLimited(chrono::Utc::now().timestamp() + 61)),
                    503 => Err(Error::ElOverblikRateLimited(chrono::Utc::now().timestamp() + 61)),
                    _ => {
                        return Ok(val);
                    }
                }
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

pub struct ClientBuilder {
    inner : Client
}

impl ClientBuilder {
    pub fn build(self) -> Client {
        self.inner
    }

    pub fn add_http(mut self, val : reqwest::Client) -> ClientBuilder {
        self.inner.http = val;
        self
    }

    pub fn add_config(mut self, val : Config) -> ClientBuilder {
        self.inner.conf = val;
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    refresh_token : String
}

pub fn new_default_client(conf : Config) -> Client {
    Client {
        http: reqwest::Client::default(),
        conf,
        data: ClientData {
            token: None
        }
    }
}

pub fn new_builder() -> ClientBuilder {
    ClientBuilder {
        inner: new_default_client(Config::default()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
