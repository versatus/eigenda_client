use crate::batch::BatchHeaderHash;
use crate::grpcurl_command;
use crate::payload::EigenDaBlobPayload;
use crate::response::BlobResponse;
use crate::status::BlobStatus;
use derive_builder::Builder;
use regex::Regex;
use std::str::FromStr;

#[derive(Builder, Clone, Debug)]
pub struct EigenDaGrpcClient {
    proto_path: String,
    server_address: String,
}

impl EigenDaGrpcClient {
    fn get_payload(&self, encoded_data: String) -> EigenDaBlobPayload {
        EigenDaBlobPayload::new(encoded_data)
    }

    pub const DISPERSE_BLOB: &'static str = "disperser.Disperser/DisperseBlob";
    pub fn disperse_blob(&self, encoded_data: String) -> Result<BlobResponse, std::io::Error> {
        let payload: String = self.get_payload(encoded_data).into();

        let output = grpcurl_command!(
            "-proto",
            &self.proto_path,
            "-d",
            &payload,
            &self.server_address,
            Self::DISPERSE_BLOB
        )?;

        if output.status.success() {
            let response: BlobResponse = String::from_utf8(output.stdout)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?
                .into();
            Ok(response)
        } else {
            let error_message = String::from_utf8(output.stderr)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ))
        }
    }

    pub const GET_BLOB_STATUS: &'static str = "disperser.Disperser/GetBlobStatus";
    pub fn get_blob_status(&self, request_id: &str) -> Result<BlobStatus, std::io::Error> {
        let payload = serde_json::json!({
            "request_id": request_id
        });

        let output = grpcurl_command!(
            "-proto",
            &self.proto_path,
            "-d",
            &payload.to_string(),
            &self.server_address,
            Self::GET_BLOB_STATUS
        )?;

        if output.status.success() {
            let response = String::from_utf8(output.stdout)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
            let re = Regex::new(r"(\\n|\\t|\n\t|\s\s+)").unwrap();
            let clean_response = re.replace_all(&response, " ").to_string();
            let res = BlobStatus::from_str(&clean_response);
            if let Err(e) = &res {
                dbg!(response);
                log::error!("{}", e);
            }
            Ok(res?)
        } else {
            let error_message = String::from_utf8(output.stderr)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ))
        }
    }

    pub const RETRIEVE_BLOB: &'static str = "disperser.Disperser/RetrieveBlob";
    pub fn retrieve_blob(
        &self,
        batch_header_hash: &BatchHeaderHash,
        blob_index: u128,
    ) -> Result<String, std::io::Error> {
        let payload = serde_json::json!({
            "batch_header_hash": batch_header_hash.to_string(),
            "blob_index": blob_index.to_string()
        });

        let output = grpcurl_command!(
            "-proto",
            &self.proto_path,
            "-d",
            &payload.to_string(),
            &self.server_address,
            Self::RETRIEVE_BLOB
        )?;

        if output.status.success() {
            let response = String::from_utf8(output.stdout)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

            Ok(response)
        } else {
            let error_message = String::from_utf8(output.stderr)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            ))
        }
    }
}
