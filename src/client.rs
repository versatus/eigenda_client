use crate::cache::LruCache;
use crate::response::BlobResponse;
use crate::payload::EigenDaBlobPayload;
use crate::status::BlobStatus;
use crate::batch::BatchHeaderHash;
use crate::grpcurl_command;

#[derive(Clone, Debug)]
pub struct EigenDaGrpcClient<C: LruCache<Value = BlobResponse>> {
    proto_path: String,
    server_address: String,
    adversary_threshold: u32,
    quorum_threshold: u32,
    blob_cache: C 
}

pub struct EigenDaGrpcClientBuilder<C: LruCache<Value = BlobResponse>> {
    proto_path: Option<String>,
    server_address: Option<String>,
    adversary_threshold: Option<u32>,
    quorum_threshold: Option<u32>,
    blob_cache: Option<C>,
}

impl<C: LruCache<Value = BlobResponse>> EigenDaGrpcClientBuilder<C> {
    pub fn new() -> Self {
        EigenDaGrpcClientBuilder { 
            proto_path: None,
            server_address: None, 
            adversary_threshold: None,
            quorum_threshold: None,
            blob_cache: None 
        }
    }

    pub fn proto_path(mut self, path: &str) -> Self {
        self.proto_path = Some(path.to_string());
        self
    }

    pub fn server_address(mut self, address: &str) -> Self {
        self.server_address = Some(address.to_string());
        self
    }

    pub fn adversary_threshold(mut self, threshold: u32) -> Self {
        self.adversary_threshold = Some(threshold);
        self
    }

    pub fn quorum_threshold(mut self, threshold: u32) -> Self {
        self.quorum_threshold = Some(threshold);
        self
    }

    pub fn blob_cache(mut self, cache: C) -> Self {
        self.blob_cache = Some(cache);
        self
    }

    pub fn build(self) -> Result<EigenDaGrpcClient<C>, String> {
        Ok(EigenDaGrpcClient {
            proto_path: self.proto_path.ok_or("proto_path is required")?,
            server_address: self.server_address.ok_or("server_address is required")?,
            adversary_threshold: self.adversary_threshold.ok_or("adversary_threshold is required")?,
            quorum_threshold: self.quorum_threshold.ok_or("quroum_threshold is required")?,
            blob_cache: self.blob_cache.ok_or("blob_cache is required")? 
        })
    }
}

impl<C: LruCache<Value = BlobResponse>> EigenDaGrpcClient<C> {
    fn get_payload(&self, raw_data: &str, quorum_id: &u32) -> EigenDaBlobPayload {
        EigenDaBlobPayload::new(  
            raw_data,
            quorum_id,
            &self.adversary_threshold,
            &self.quorum_threshold,
        ) 
    }

    pub fn disperse_blob(&mut self, raw_data: &str, quorum_id: &u32) -> Result<BlobResponse, std::io::Error> {
        let payload: String = self.get_payload(raw_data, quorum_id).into();

        let output = grpcurl_command!(
            "-proto", &self.proto_path,
            "-d", &payload,
            &self.server_address,
            "disperser.Disperser/DisperseBlob"
        )?;

        if output.status.success() {
            let response: BlobResponse = String::from_utf8(output.stdout).map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::Other, err.to_string()
                )
            })?.into();
            self.blob_cache.cache(&response);
            Ok(response)
        } else {
            let error_message = String::from_utf8(output.stderr).map_err(|err| {
                    std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
                }
            )?;
            Err(std::io::Error::new(std::io::ErrorKind::Other, error_message))
        }
    }

    pub fn get_blob_status(&mut self, request_id: &str) -> Result<BlobStatus, std::io::Error> {
        let mut payload = String::new();
        payload.push_str(r#"{"#);
        payload.push_str(r#""request_id":"#);
        payload.push_str(&format!(r#""{}""#, request_id));
        payload.push_str(r#"}"#);

        let output = grpcurl_command!(
            "-proto", &self.proto_path,
            "-d", &payload,
            &self.server_address,
            "disperser.Disperser/GetBlobStatus"
        )?;

        if output.status.success() {
            let response = String::from_utf8(output.stdout).map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            })?.into();

            Ok(response)
        } else {
            let error_message = String::from_utf8(output.stderr).map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            })?;
            Err(std::io::Error::new(std::io::ErrorKind::Other, error_message))
        }
    }

    pub fn retrieve_blob(&mut self, batch_header_hash: &BatchHeaderHash) -> Result<String, std::io::Error> {
        let mut payload = String::new();
        payload.push_str(r#"{"#);
        payload.push_str(r#""batch_header_hash":"#);
        payload.push_str(&format!(r#""{}""#, batch_header_hash.to_string()));
        payload.push_str(r#"}"#);

        let output = grpcurl_command!(
            "-proto", &self.proto_path,
            "-d", &payload,
            &self.server_address,
            "disperser.Disperser/RetrieveBlob"
        )?;

        if output.status.success() {
            let response = String::from_utf8(output.stdout).map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            })?;

            Ok(response)
        } else {
            let error_message = String::from_utf8(output.stderr).map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            })?;

            Err(std::io::Error::new(std::io::ErrorKind::Other, error_message))
        }
    }
}
