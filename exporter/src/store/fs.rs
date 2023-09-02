use std::io::Write;
use crate::store::{Store, StoreType};
use crate::error::Result;

pub struct FsStore {
    pub path : String
}

impl Store for FsStore {
    fn put(&self, doc: StoreType) -> Result<()> {
        let mut file_name : String;
        let mut content : Vec<u8> = Vec::new();

        match &doc {
            StoreType::String { key, value } => {
                file_name = key.clone();
                content = value.as_bytes().to_vec();
            }
            StoreType::MeterDataTimeSeries(resp) => {
                file_name = resp.id.clone();
                content = serde_json::to_vec(resp).unwrap();
            }
        }

        let mut path_buf = std::path::PathBuf::new();
        path_buf.push(&self.path);
        std::fs::create_dir_all(path_buf.as_path()).unwrap();
        path_buf.push(file_name);

        let mut file = std::fs::File::create(path_buf.as_path()).unwrap();
        file.write_all(&content).unwrap();

        Ok(())
    }
}