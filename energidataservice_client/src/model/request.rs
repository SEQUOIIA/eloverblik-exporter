use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElSpotPricesRequest {
    pub limit: Option<i32>,
    pub timezone: Option<String>,
    pub start : Option<String>,
    pub end : Option<String>,
    pub filter : Option<String>,
    pub sort : Option<String>,
}

impl ElSpotPricesRequest {
    pub fn tuples(self) -> Vec<(String, String)> {
        let mut payload = Vec::new();

        if let Some(val) = self.limit {
            payload.push(("limit".to_owned(), val.to_string()));
        }

        if let Some(val) = self.timezone {
            payload.push(("timezone".to_owned(), val));
        }

        if let Some(val) = self.start {
            payload.push(("start".to_owned(), val));
        }

        if let Some(val) = self.end {
            payload.push(("end".to_owned(), val));
        }

        if let Some(val) = self.filter {
            payload.push(("filter".to_owned(), val));
        }

        if let Some(val) = self.sort {
            payload.push(("sort".to_owned(), val));
        }

        payload
    }
}