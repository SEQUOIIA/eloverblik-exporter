use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct CString {
    val : String
}

impl Serialize for CString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.val.as_str())
    }
}

impl<'de> Deserialize<'de> for CString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(Self {
            val: s
        })
    }
}

impl From<Vec<u8>> for CString {
    fn from(value: Vec<u8>) -> Self {
        let val = String::from_utf8(value).unwrap();
        Self {
            val
        }
    }
}

impl From<String> for CString {
    fn from(value: String) -> Self {
        Self {
            val: value
        }
    }
}

impl From<CString> for String {
    fn from(value: CString) -> Self {
        value.val
    }
}

impl AsRef<[u8]> for CString {
    fn as_ref(&self) -> &[u8] {
        self.val.as_bytes()
    }
}
