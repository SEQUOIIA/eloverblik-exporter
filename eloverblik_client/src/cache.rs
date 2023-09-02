use std::collections::{HashMap};
use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};
use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait Cache<T> {
    fn put(&mut self, key : &str, val : T, expiration_time : Option<i64>);
    fn get(&self, key : &str) -> Option<T>;
    fn has_expired(&self, key : &str) -> bool;
}

impl<T> Debug for dyn Cache<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct InMemoryCache<T> {
    store : HashMap<String, T>,
    expiration : HashMap<String, i64>,
    default_expiration_time_in_secs : i64
}

impl<T : Clone> Cache<T> for InMemoryCache<T> {
    fn put(&mut self, key: &str, val: T, expiration_time : Option<i64>) {
        match expiration_time {
            None => {
                self.expiration.insert(key.to_owned(), chrono::Utc::now().timestamp() + self.default_expiration_time_in_secs);
            }
            Some(timestamp) => {
                self.expiration.insert(key.to_owned(), timestamp);
            }
        }
        self.store.insert(key.to_owned(), val);
    }

    fn get(&self, key: &str) -> Option<T> {
        return match self.store.get(key) {
            None => None,
            Some(val) => Some(val.clone())
        }
    }

    fn has_expired(&self, key: &str) -> bool {
        return match self.expiration.get(key) {
            None => true,
            Some(timestamp) => chrono::Utc::now().timestamp() > timestamp.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiskCache {
    pub path : String,
    pub default_expiration_time_in_secs : i64
}


#[derive(Clone, Serialize, Deserialize)]
struct DiskCacheStructure<T> {
    pub expires_in : i64,
    pub data : T
}
impl<'b, T : AsRef<[u8]> + for<'a> Deserialize<'a> + From<Vec<u8>>> Cache<T> for DiskCache {
    fn put(&mut self, key: &str, val: T, expiration_time: Option<i64>) {
        let expr_time = match expiration_time {
            None => {
                self.default_expiration_time_in_secs + chrono::Utc::now().timestamp()
            }
            Some(val) => {
                val
            }
        };

        let payload = DiskCacheStructure {
            expires_in: expr_time,
            data: base64::engine::general_purpose::STANDARD.encode(val),
        };

        let mut path_buf = std::path::PathBuf::new();
        path_buf.push(&self.path);
        std::fs::create_dir_all(path_buf.as_path()).unwrap();
        path_buf.push(key);
        let mut file = std::fs::File::create(path_buf.as_path()).unwrap();
        file.write_all(serde_json::to_vec(&payload).unwrap().as_slice()).unwrap();
    }

    fn get(&self, key: &str) -> Option<T> {
        let mut path_buf = std::path::PathBuf::new();
        path_buf.push(&self.path);
        path_buf.push(key);
        let mut file = std::fs::File::open(path_buf.as_path());

        return match file {
            Ok(mut f) => {
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).unwrap();
                let obj : DiskCacheStructure<T> = serde_json::from_slice(&buf).unwrap();
                let decoded = base64::engine::general_purpose::STANDARD.decode(obj.data).unwrap();
                Some(decoded.into())
            }
            Err(_) => None
        };
    }

    fn has_expired(&self, key: &str) -> bool {
        let mut path_buf = std::path::PathBuf::new();
        path_buf.push(&self.path);
        path_buf.push(key);
        let file = std::fs::File::open(path_buf.as_path());

        return match file {
            Ok(mut f) => {
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).unwrap();
                let obj : DiskCacheStructure<T> = serde_json::from_slice(&buf).unwrap();
                chrono::Utc::now().timestamp() > obj.expires_in
            }
            Err(_) => true
        };
    }
}