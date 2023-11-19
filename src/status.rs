use serde::{Serialize, Deserialize};
use crate::info::BlobInfo;
use crate::quorum::{
    BlobQuorumParams,
    BlobQuorumIndexes,
    BlobQuorumNumbers,
    BlobQuorumSignedPercentages
};
use crate::batch::{BlobBatchRoot, BatchHeader, BatchHeaderHash};
use crate::header::BlobHeader;
use crate::proof::{BlobInclusionProof, BlobVerificationProof};
use crate::commitment::BlobCommitment;
use crate::meta::BatchMetadata;
use crate::record::BlobSignatoryRecordHash;
use crate::fee::BlobFee;

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
