use crate::cache::LruCache;
use crate::response::BlobResponse;
use ritelinked::LinkedHashSet;

pub mod batch;
pub mod blob;
pub mod cache;
pub mod client;
pub mod commitment;
pub mod error;
pub mod fee;
pub mod header;
pub mod info;
pub mod macros;
pub mod meta;
pub mod methods;
pub mod payload;
pub mod proof;
pub mod quorum;
pub mod record;
pub mod response;
pub mod result;
pub mod status;

pub use client::*;

impl LruCache for LinkedHashSet<BlobResponse> {
    type Value = BlobResponse;

    fn cache(&mut self, item: &Self::Value) {
        self.insert(item.clone());
    }

    fn get(&self, key: &Self::Item) -> Option<&Self::Value> {
        self.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use crate::blob::{DecodedBlob, EncodedBlob};
    use crate::client::EigenDaGrpcClient;
    use crate::status::BlobResult;
    use std::thread;
    use std::time::Duration;

    #[test]
    #[ignore = "encountered an error to convert a 32-bytes into a valid field element, please use the correct format where every 32bytes(big-endian) is less than 21888242871839275222246405745257275088548364400416034343698204186575808495617"]
    fn test_disperse_get_status_and_retrieve_blob() {
        let client = create_client();
        let arbitrary_data = base64::encode("ArbitraryData");

        let blob_response = client.disperse_blob(arbitrary_data).unwrap();
        let mut blob_status = client.get_blob_status(&blob_response.request_id()).unwrap();
        while blob_status.status() != &BlobResult::Confirmed {
            thread::sleep(Duration::from_secs(30));
            blob_status = client.get_blob_status(&blob_response.request_id()).unwrap();
        }

        let batch_header_hash = blob_status.batch_header_hash().unwrap();
        let blob_index = blob_status.blob_index().unwrap();

        let blob = client.retrieve_blob(batch_header_hash, blob_index).unwrap();

        let blob = EncodedBlob::from_str(&blob).unwrap();

        let decoded_blob = DecodedBlob::from_encoded(blob).unwrap();
        println!("{}", decoded_blob.len());
    }

    fn create_client() -> EigenDaGrpcClient {
        EigenDaGrpcClient::default()
    }
}
