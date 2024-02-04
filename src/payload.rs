use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct EigenDaBlobPayload {
    data: String,
    quorum_id: u32,
    adversary_threshold: u32,
    quorum_threshold: u32,
}

impl EigenDaBlobPayload {
    pub fn new(
        data: String,
        quorum_id: &u32,
        adversary_threshold: &u32, 
        quorum_threshold: &u32,
    ) -> Self {
        EigenDaBlobPayload { 
            data,
            quorum_id: *quorum_id,
            adversary_threshold: *adversary_threshold,
            quorum_threshold: *quorum_threshold,
        }
    }
}

impl From<EigenDaBlobPayload> for String {
    fn from(value: EigenDaBlobPayload) -> Self {
        let payload = serde_json::json!({
            "data": value.data,
            "security_params": [{
                "quorum_id": value.quorum_id.to_string(),
                "adversary_threshold": value.adversary_threshold.to_string(),
                "quorum_threshold": value.quorum_threshold.to_string()
            }]
        });
        payload.to_string()
    }
}

impl From<&EigenDaBlobPayload> for String {
    fn from(value: &EigenDaBlobPayload) -> Self {
        let payload = serde_json::json!({
            "data": value.data,
            "security_params": [{
                "quorum_id": value.quorum_id.to_string(),
                "adversary_threshold": value.adversary_threshold.to_string(),
                "quorum_threshold": value.quorum_threshold.to_string()
            }]
        });
        payload.to_string()
    }
}
