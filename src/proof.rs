use serde::{Serialize, Deserialize};
use crate::meta::BatchMetadata;
use crate::quorum::BlobQuorumIndexes;

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
