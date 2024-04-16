use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct EigenDaBlobPayload {
    data: String,
}

impl EigenDaBlobPayload {
    pub fn new(
        data: String,
    ) -> Self {
        EigenDaBlobPayload { 
            data
        }
    }
}

impl From<EigenDaBlobPayload> for String {
    fn from(value: EigenDaBlobPayload) -> Self {
        let payload = serde_json::json!({
            "data": value.data
        });
        payload.to_string()
    }
}

impl From<&EigenDaBlobPayload> for String {
    fn from(value: &EigenDaBlobPayload) -> Self {
        let payload = serde_json::json!({
            "data": value.data
        });
        payload.to_string()
    }
}
