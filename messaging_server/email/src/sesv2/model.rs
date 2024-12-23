use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BulkEamilEntriesSESV2 {
    pub to_email: String,
    pub replacement_template_json: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendBulkEmailRequestDTO {
    pub from_email: String,
    pub to_email_and_content: Vec<BulkEamilEntriesSESV2>,
    pub content_subject: String,
    pub content_html: String,
}
