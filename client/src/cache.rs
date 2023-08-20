use std::collections::{HashMap};

pub trait Cache<T> {
    fn put(&mut self, key : &str, val : T, expiration_time : Option<i64>);
    fn get(&self, key : &str) -> Option<&T>;
    fn has_expired(&self, key : &str) -> bool;
}

pub struct InMemoryCache<T> {
    store : HashMap<String, T>,
    expiration : HashMap<String, i64>,
    default_expiration_time_in_secs : i64
}

impl<T> Cache<T> for InMemoryCache<T> {
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

    fn get(&self, key: &str) -> Option<&T> {
        self.store.get(key)
    }

    fn has_expired(&self, key: &str) -> bool {
        return match self.expiration.get(key) {
            None => true,
            Some(timestamp) => chrono::Utc::now().timestamp() > timestamp.clone()
        }
    }
}