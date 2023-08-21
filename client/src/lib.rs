pub mod error;
pub mod model;
pub mod cache;
pub mod types;

use std::sync::Arc;
use crossbeam::sync::ShardedLock;
use log::debug;
use reqwest::{Request, Response};
use serde::{Deserialize, Serialize};
use error::{Result, Error};
use crate::cache::{Cache};
use crate::types::cstring::CString;

const BASE_URL : &'static str = "https://api.eloverblik.dk/customerapi";
const CACHE_KEY : &'static str = "ACCESS_TOKEN";

#[derive(Clone, Debug)]
pub struct Client {
    http : reqwest::Client,
    conf : Config,
    cache : Option<Arc<ShardedLock<Box<dyn Cache<CString>>>>>,
    data : ClientData
}

#[derive(Clone, Debug)]
struct ClientData {
    pub token : Option<Arc<ShardedLock<Token>>>
}

#[derive(Clone, Debug)]
struct Token {
    pub token : String,
    pub expires_in : i64
}

// Limit most requests to 25 queries every 60 seconds
impl Client {

    // Limit to 2 queries every 60 seconds
    pub async fn auth(&self) -> Result<model::response::TokenResponse> {
        let mut req = Request::new(reqwest::Method::GET, format!("{}/api/token", BASE_URL).parse().unwrap());
        req.headers_mut().insert("Authorization", format!("Bearer {}", self.conf.refresh_token).parse().unwrap());

        let resp = self.check_response(self.http.execute(req).await);

        match resp {
            Ok(val) => {
                let deserialised : Result<model::response::TokenResponse> = val.json().await.map_err(|err | err.into());
                return deserialised;
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    async fn store_token(&self, token : String) {
        if let Some(ch) = self.cache.as_ref() {
            let mut write_lock = ch.write().unwrap();
            write_lock.put(CACHE_KEY, token.into(), None)
        }
    }

    async fn prepare_http_request(&self, req : &mut Request) {
        let mut token = "".to_owned();
        match &self.cache {
            None => {
                debug!(target:"eloverblik_client::auth", "Skipping cache, getting new token");
                let resp = self.auth().await.unwrap();
                token = resp.result;
            }
            Some(ch) => {
                let mut get_new_token = false;
                {
                    let lock = ch.read().unwrap();
                    get_new_token = lock.has_expired(CACHE_KEY);
                }

                if get_new_token {
                    debug!(target:"eloverblik_client::auth", "Skipping cache, getting new token");
                    let resp = self.auth().await.unwrap();
                    token = resp.result.clone();
                    // token = "dummy".to_owned();
                    let mut write_lock = ch.write().unwrap();
                    write_lock.put(CACHE_KEY, token.clone().into(), None);
                } else {
                    debug!(target:"eloverblik_client::auth", "Using cache");
                    let lock = ch.read().unwrap();
                    token = lock.get(CACHE_KEY).unwrap().clone().into();
                }
            }
        }

        req.headers_mut().insert("Authorization", format!("Bearer {}", token).parse().unwrap());
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

    pub async fn get_metering_points(&self) {
        let mut req = Request::new(reqwest::Method::GET, format!("{}/api/meteringpoints/meteringpoints", BASE_URL).parse().unwrap());
        self.prepare_http_request(&mut req).await;
        let resp = self.http.execute(req).await.unwrap();

        println!("{}", resp.text().await.unwrap());
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

    pub fn add_cache(mut self, val : Box<dyn Cache<CString>>) -> ClientBuilder {
        self.inner.cache = Some(Arc::new(ShardedLock::new(val)));
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub refresh_token : String
}

pub fn new_default_client(conf : Config) -> Client {
    Client {
        http: reqwest::Client::default(),
        conf,
        cache: None,
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
