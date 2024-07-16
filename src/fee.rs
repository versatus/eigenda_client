use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobFee(String);

impl ToString for BlobFee {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
