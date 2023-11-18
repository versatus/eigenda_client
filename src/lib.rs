use std::hash::Hash;
use ritelinked::LinkedHashSet;
use serde::{Serialize, Deserialize};

pub trait LruCache: IntoIterator {
    type Value: Hash;

    fn cache(&mut self, item: &Self::Value); 
    //TODO: Add a method to clean the cache at a configurable duration
    //fn clean(&mut self, n: usize);
}

#[macro_export]
macro_rules! grpcurl_command {
    ($($arg:expr),*) => {{
        let mut command = std::process::Command::new("grpcurl");
        $(
            command.arg($arg);
        )*
        command.output() 
        // This returns a Result<std::process::Output, std::io::Error>
    }};
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum EigenDaGrpcMethod {
    DisperseBlob,
    GetBlobStatus,
}

impl ToString for EigenDaGrpcMethod {
    fn to_string(&self) -> String {
        match self {
            EigenDaGrpcMethod::DisperseBlob => {
                return "disperser.Disperser/DisperseBlob".to_string()
            },
            EigenDaGrpcMethod::GetBlobStatus => {
                return "disperser.Disperser/GetBlobStatus".to_string()
            }
        }
    }
}

// TODO: Implement custom Deserialize
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BlobResult {
    Processing,
    Confirmed,
    Failed,
    Other(String)
}

impl Default for BlobResult {
    fn default() -> Self {
        BlobResult::Other("Default".to_string())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlobResponse {
    result: BlobResult,
    #[serde(rename = "requestId")]
    request_id: String
}

impl Default for BlobResponse {
    fn default() -> Self {
        BlobResponse { result: Default::default(), request_id: Default::default() }
    }
}

impl From<String> for BlobResponse {
    fn from(value: String) -> Self {
        if let Some(start_index) = value.find('{') {
            let json_str = &value[start_index..];
            println!("{}", &json_str);

            let blob_response: Result<BlobResponse, serde_json::Error> = serde_json::from_str(json_str);

            match blob_response {
                Ok(response) => {
                    return response
                }
                Err(err) => {
                    println!("{}", &err);
                    return BlobResponse::default() 
                }
            }
        } else {
            return BlobResponse::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlobStatus {
    status: BlobResult,
    info: BlobInfo
}

impl BlobStatus {
    pub fn status(&self) -> &BlobResult {
        &self.status
    }

    pub fn info(&self) -> &BlobInfo {
        &self.info
    }
    
    pub fn blob_header(&self) -> &BlobHeader {
        &self.info().blob_header()
    }

    pub fn blob_verification_proof(&self) -> &BlobVerificationProof {
        &self.info().blob_verification_proof()
    }

    pub fn commitment(&self) -> &BlobCommitment {
        &self.blob_header().commitment()
    }

    pub fn data_length(&self) -> usize {
        self.blob_header().data_length()
    }

    pub fn blob_quorum_params(&self) -> &Vec<BlobQuorumParams> {
        &self.blob_header().blob_quorum_params()
    }

    pub fn batch_id(&self) -> u128 {
        self.blob_verification_proof().batch_id()
    }

    pub fn blob_index(&self) -> u128 {
        self.blob_verification_proof().blob_index()
    }

    pub fn batch_metadata(&self) -> &BatchMetadata {
        self.blob_verification_proof().batch_metadata()
    }

    pub fn inclusion_proof(&self) -> &BlobInclusionProof {
        self.blob_verification_proof().inclusion_proof()
    }

    pub fn quorum_indexes(&self) -> &BlobQuorumIndexes {
        self.blob_verification_proof().quorum_indexes()
    }

    pub fn batch_header(&self) -> &BatchHeader {
        self.batch_metadata().batch_header()
    }

    pub fn signatory_record_hash(&self) -> &BlobSignatoryRecordHash {
        self.batch_metadata().signatory_record_hash()
    }

    pub fn fee(&self) -> &BlobFee {
        self.batch_metadata().fee()
    }

    pub fn confirmation_block_number(&self) -> u128 {
        self.batch_metadata().confirmation_block_number()
    }

    pub fn batch_header_hash(&self) -> &BatchHeaderHash {
        self.batch_metadata().batch_header_hash()
    }
    
    pub fn batch_root(&self) -> &BlobBatchRoot {
        self.batch_header().batch_root()
    }

    pub fn quorum_numbers(&self) -> &BlobQuorumNumbers {
        self.batch_header().quorum_numbers()
    }

    pub fn quorum_signed_percentages(&self) -> &BlobQuorumSignedPercentages {
        self.batch_header().quorum_signed_percentages()
    }

    pub fn reference_block_number(&self) -> u128 {
        self.batch_header().reference_block_number()
    }
}

impl From<String> for BlobStatus {
    fn from(value: String) -> Self {
        if let Some(start_index) = value.find('{') {
            let json_str = &value[start_index..];
            let blob_status: Result<BlobStatus, serde_json::Error> = serde_json::from_str(json_str);
            match blob_status {
                Ok(status) => {
                    return status
                }
                Err(_) => {
                    return BlobStatus::default()
                }
            }
        } else {
            return BlobStatus::default()
        }
    }
}

impl Default for BlobStatus {
    fn default() -> Self {
        BlobStatus {
            status: Default::default(),
            info: Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobInfo {
    blob_header: BlobHeader,
    blob_verification_proof: BlobVerificationProof,
}

impl BlobInfo {
    pub fn blob_header(&self) -> &BlobHeader {
        &self.blob_header
    }

    pub fn blob_verification_proof(&self) -> &BlobVerificationProof {
        &self.blob_verification_proof
    }
}

impl Default for BlobInfo {
    fn default() -> Self {
        BlobInfo {
            blob_header: Default::default(),
            blob_verification_proof: Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobHeader {
    commitment: BlobCommitment,
    data_length: usize,
    blob_quorum_params: Vec<BlobQuorumParams>,
}

impl BlobHeader {
    pub fn commitment(&self) -> & BlobCommitment {
        &self.commitment
    }

    pub fn data_length(&self) -> usize {
        self.data_length
    }

    pub fn blob_quorum_params(&self) -> &Vec<BlobQuorumParams> {
        &self.blob_quorum_params
    }
}

impl Default for BlobHeader {
    fn default() -> Self {
        BlobHeader { 
            commitment: Default::default(),
            data_length: Default::default(),
            blob_quorum_params: Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobCommitment(String);

impl ToString for BlobCommitment {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobVerificationProof {
    batch_id: u128,
    blob_index: u128,
    batch_metadata: BatchMetadata,
    inclusion_proof: BlobInclusionProof, 
    quorum_indexes: BlobQuorumIndexes, 
}

impl BlobVerificationProof {
    pub fn batch_id(&self) -> u128 {
        self.batch_id
    }

    pub fn blob_index(&self) -> u128 {
        self.blob_index
    }

    pub fn batch_metadata(&self) -> &BatchMetadata {
        &self.batch_metadata
    }

    pub fn inclusion_proof(&self) -> &BlobInclusionProof {
        &self.inclusion_proof
    }

    pub fn quorum_indexes(&self) -> &BlobQuorumIndexes {
        &self.quorum_indexes
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobInclusionProof(String);

impl ToString for BlobInclusionProof {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobQuorumIndexes(String);

impl ToString for BlobQuorumIndexes {
    fn to_string(&self) -> String {
       self.0.clone() 
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobQuorumParams {
    adversary_threshold_percentage: usize,
    quorum_threshold_percentage: usize,
    quantization_param: usize,
    encoded_length: String,
}

impl BlobQuorumParams {
    pub fn adversary_threshold_percentage(&self) -> usize {
        self.adversary_threshold_percentage
    }

    pub fn quorum_threshold_percentage(&self) -> usize {
        self.quorum_threshold_percentage
    }

    pub fn quantization_param(&self) -> usize {
        self.quantization_param
    }

    pub fn encoded_length(&self) -> String {
        self.encoded_length.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchMetadata {
    batch_header: BatchHeader,
    signatory_record_hash: BlobSignatoryRecordHash,
    fee: BlobFee,
    confirmation_block_number: u128,
    batch_header_hash: BatchHeaderHash 
}

impl BatchMetadata {
    pub fn batch_header(&self) -> &BatchHeader {
        &self.batch_header
    }

    pub fn signatory_record_hash(&self) -> &BlobSignatoryRecordHash {
        &self.signatory_record_hash
    }

    pub fn fee(&self) -> &BlobFee {
        &self.fee
    }

    pub fn confirmation_block_number(&self) -> u128 {
        self.confirmation_block_number
    }

    pub fn batch_header_hash(&self) -> &BatchHeaderHash {
        &self.batch_header_hash
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobSignatoryRecordHash(String);

impl ToString for BlobSignatoryRecordHash {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobFee(String);

impl ToString for BlobFee { 
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchHeaderHash(String);

impl ToString for BatchHeaderHash {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchHeader {
    batch_root: BlobBatchRoot,
    quorum_numbers: BlobQuorumNumbers,
    quorum_signed_percentages: BlobQuorumSignedPercentages,
    reference_block_number: u128
}

impl BatchHeader {
    pub fn batch_root(&self) -> &BlobBatchRoot {
        &self.batch_root
    }

    pub fn quorum_numbers(&self) -> &BlobQuorumNumbers {
        &self.quorum_numbers
    }

    pub fn quorum_signed_percentages(&self) -> &BlobQuorumSignedPercentages {
        &self.quorum_signed_percentages
    }

    pub fn reference_block_number(&self) -> u128 {
        self.reference_block_number
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobBatchRoot(String);

impl ToString for BlobBatchRoot {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobQuorumNumbers(String);

impl ToString for BlobQuorumNumbers {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlobQuorumSignedPercentages(String);

impl ToString for BlobQuorumSignedPercentages {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}


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

#[derive(Clone, Debug)]
pub struct EigenDaBlobPayload {
    data: String,
    quorum_id: u32,
    adversary_threshold: u32,
    quorum_threshold: u32,
}

impl From<EigenDaBlobPayload> for String {
    fn from(value: EigenDaBlobPayload) -> Self {
        let mut json_payload = String::new();
        json_payload.push_str(r#"{"data":"#);
        json_payload.push_str(&format!(r#""{}""#, &value.data));
        json_payload.push_str(r#", "security_params":[{"quorum_id":"#);
        json_payload.push_str(&value.quorum_id.to_string());
        json_payload.push_str(r#","adversary_threshold":"#);
        json_payload.push_str(&value.adversary_threshold.to_string());
        json_payload.push_str(r#","quorum_threshold":"#);
        json_payload.push_str(&value.quorum_threshold.to_string());
        json_payload.push_str(r#"}]}"#);

        json_payload
        
    }
}

impl From<&EigenDaBlobPayload> for String {
    fn from(value: &EigenDaBlobPayload) -> Self {
        let mut json_payload = String::new();

        json_payload.push_str(r#"{"data":"#);
        json_payload.push_str(&format!(r#""{}""#, &value.data));
        json_payload.push_str(r#", "security_params":[{"quorum_id":"#);
        json_payload.push_str(&value.quorum_id.to_string());
        json_payload.push_str(r#"","adversary_threshold":"#);
        json_payload.push_str(&value.adversary_threshold.to_string());
        json_payload.push_str(r#","quorum_threshold":"#);
        json_payload.push_str(&value.quorum_threshold.to_string());
        json_payload.push_str(r#"}]}"#);

        json_payload
        
    }
}

impl<C: LruCache<Value = BlobResponse>> EigenDaGrpcClient<C> {

    fn get_payload(&self, raw_data: &str, quorum_id: u32) -> EigenDaBlobPayload {
        let data = base64::encode(raw_data);
        EigenDaBlobPayload { 
            data,
            quorum_id,
            adversary_threshold: self.adversary_threshold,
            quorum_threshold: self.quorum_threshold,
        }
    }

    pub fn disperse_blob(&mut self, raw_data: &str, quorum_id: u32) -> Result<BlobResponse, std::io::Error> {
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

impl LruCache for LinkedHashSet<BlobResponse> {
    type Value = BlobResponse;

    fn cache(&mut self, item: &Self::Value) {
        self.insert(item.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_disperse_get_status_and_retrieve_blob() {

        let mut client = create_client(40, 60); 
        let arbitrary_data = "ArbitraryData";
        
        let blob_response = client.disperse_blob(arbitrary_data, 0).unwrap();
        let mut blob_status = client.get_blob_status(&blob_response.request_id).unwrap(); 
        while blob_status.status() != &BlobResult::Confirmed {
            thread::sleep(Duration::from_secs(30));
            blob_status = client.get_blob_status(&blob_response.request_id).unwrap();
        }

        let batch_header_hash = blob_status.batch_header_hash();

        let blob = client.retrieve_blob(batch_header_hash);

        println!("{:?}", blob);
    }

    fn create_client(adversary_threshold: u32, quorum_threshold: u32) -> EigenDaGrpcClient<LinkedHashSet<BlobResponse>> 
    {
        EigenDaGrpcClientBuilder::new()
            .proto_path("./eigenda/api/proto/disperser/disperser.proto")
            .server_address("disperser-goerli.eigenda.xyz:443")
            .adversary_threshold(adversary_threshold)
            .quorum_threshold(quorum_threshold)
            .blob_cache(LinkedHashSet::new())
            .build()
            .unwrap()
    } 
}
