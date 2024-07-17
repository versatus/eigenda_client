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
    disperser_path: String,
    server_address: String,
}

pub const DEFAULT_EIGENDA_SERVER_ADDRESS: &str = "disperser-holesky.eigenda.xyz:443";
pub const EIGENDA_PROTO_PATH: &str = "eigenda/api/proto";
pub const EIGENDA_COMMON_PROTO_FILENAME: &str = "common.proto";
pub const EIGENDA_DISPERSER_PROTO_FILENAME: &str = "disperser.proto";
impl Default for EigenDaGrpcClient {
    fn default() -> Self {
        let mut eigenda_proto_path = std::env::current_dir()
            .expect("failed to get current directory for checking eigenda api path.");
        eigenda_proto_path.push(EIGENDA_PROTO_PATH);

        let mut common_path = eigenda_proto_path.clone();
        common_path.push("common");
        if !common_path.exists() {
            std::fs::create_dir_all(&common_path)
                .expect("failed to create eigenda common proto path.");
        }
        common_path.push(EIGENDA_COMMON_PROTO_FILENAME);
        std::fs::write(
            &common_path,
            include_bytes!("../eigenda/api/proto/common/common.proto"),
        )
        .expect("failed to write eigenda common proto api to file.");

        let mut disperser_path = eigenda_proto_path.clone();
        disperser_path.push("disperser");
        if !disperser_path.exists() {
            std::fs::create_dir_all(&disperser_path)
                .expect("failed to create eigenda disperser proto path.");
        }
        disperser_path.push(EIGENDA_DISPERSER_PROTO_FILENAME);
        std::fs::write(
            &disperser_path,
            include_bytes!("../eigenda/api/proto/disperser/disperser.proto"),
        )
        .expect("failed to write eigenda disperser proto api to file.");

        EigenDaGrpcClientBuilder::default()
            .proto_path(
                eigenda_proto_path
                    .to_str()
                    .expect("failed to convert eigenda proto path to &str")
                    .to_string(),
            )
            .disperser_path(
                disperser_path
                    .to_str()
                    .expect("failed to convert eigenda disperser proto path to &str")
                    .to_string(),
            )
            .server_address(DEFAULT_EIGENDA_SERVER_ADDRESS.to_string())
            .build()
            .expect("failed to build eigenda gRPC client.")
    }
}

impl EigenDaGrpcClient {
    /// Update the EigenDA server address with some URL address other than [`DEFAULT_EIGENDA_SERVER_ADDRESS`].
    pub fn update_server_address(&mut self, address: String) {
        self.server_address = address;
    }

    fn get_payload(&self, encoded_data: String) -> EigenDaBlobPayload {
        EigenDaBlobPayload::new(encoded_data)
    }

    pub const DISPERSE_BLOB: &'static str = "disperser.Disperser/DisperseBlob";
    pub fn disperse_blob(&self, encoded_data: String) -> Result<BlobResponse, std::io::Error> {
        let payload: String = self.get_payload(encoded_data).into();

        let output = grpcurl_command!(
            "-import-path",
            &self.proto_path,
            "-proto",
            &self.disperser_path,
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
            "-import-path",
            &self.proto_path,
            "-proto",
            &self.disperser_path,
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
            "-import-path",
            &self.proto_path,
            "-proto",
            &self.disperser_path,
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
