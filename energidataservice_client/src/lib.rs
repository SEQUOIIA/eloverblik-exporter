pub mod error;
pub mod model;
pub mod cache;
pub mod types;

use std::sync::Arc;
use crossbeam::sync::ShardedLock;
use reqwest::{Request, Response, Url};
use serde::{Deserialize, Serialize};
use error::{Result, Error};
use crate::cache::{Cache};
use crate::model::request::{ElSpotPricesRequest};
use crate::model::response::ElSpotPricesResponse;
use crate::types::cstring::CString;

const BASE_URL : &'static str = "https://api.energidataservice.dk";

#[derive(Clone, Debug)]
pub struct Client {
    http : reqwest::Client,
    conf : Config,
    cache : Option<Arc<ShardedLock<Box<dyn Cache<CString>>>>>,
    data : ClientData
}

#[derive(Clone, Debug)]
struct ClientData {
}

// Limit most requests to 25 queries every 60 seconds
impl Client {
    async fn prepare_http_request(&self, req : &mut Request) {

    }

    // Handle rate limits
    fn check_response(&self, resp : std::result::Result<Response, reqwest::Error>) -> Result<Response> {
        match resp {
            Ok(val) => {
                match val.status().as_u16() {
                    429 => Err(Error::RateLimited(chrono::Utc::now().timestamp() + 61)),
                    503 => Err(Error::RateLimited(chrono::Utc::now().timestamp() + 61)),
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

    pub async fn get_elspotprices(&self, params : ElSpotPricesRequest) -> Result<ElSpotPricesResponse> {
        let mut req = self.http.get(format!("{}/dataset/Elspotprices", BASE_URL).parse::<Url>().unwrap())
            .query(params.tuples().as_slice())
            .build().unwrap();
        self.prepare_http_request(&mut req).await;

        let resp = self.http.execute(req).await;
        let checked_resp = self.check_response(resp).unwrap();

        checked_resp.json().await.map_err(|err| err.into())
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
}

pub fn new_default_client(conf : Config) -> Client {
    Client {
        http: reqwest::Client::default(),
        conf,
        cache: None,
        data: ClientData {
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
