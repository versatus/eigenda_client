use serde::{Serialize, Deserialize};
use crate::proof::BlobVerificationProof;
use crate::header::BlobHeader;

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
