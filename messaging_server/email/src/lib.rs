pub mod sesv2 {
    pub mod client;
    pub mod email_logger;
    pub mod model;
    pub mod send_bulk_email;
}

use aws_sdk_sesv2::operation::send_bulk_email::SendBulkEmailError;
use aws_sdk_sesv2::types::BulkEmailEntry;
use std::env;

use sesv2::client::get_aws_client;
use sesv2::model::BulkEamilEntriesSESV2;
use sesv2::send_bulk_email::{convert_email_content, convert_email_entries, send_bulk_email};

pub async fn send_bulk_email_sesv2(
    bulk_email_entries: Vec<BulkEamilEntriesSESV2>,
    from_email: String,
    content_subject: String,
    content_html: String,
) -> Result<String, SendBulkEmailError> {
    let access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("No aws access key in ENV");
    let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("No aws access key in ENV");

    let sesv2_client = get_aws_client(&access_key_id, &secret_access_key).await;

    let formatted_bulk_eamil_entries: Vec<BulkEmailEntry> =
        convert_email_entries(bulk_email_entries);

    let formatted_content = convert_email_content(content_subject, content_html);

    return send_bulk_email(
        sesv2_client,
        from_email,
        formatted_bulk_eamil_entries,
        formatted_content,
    )
    .await;
}
