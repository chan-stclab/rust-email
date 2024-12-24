use super::email_logger::make_log_file_of_sesv2_response;
use std::{collections::HashMap, time};

use super::model::BulkEamilEntriesSESV2;

use aws_sdk_sesv2::{
    operation::send_bulk_email::SendBulkEmailError,
    types::{
        BulkEmailContent, BulkEmailEntry, BulkEmailStatus, Destination, EmailTemplateContent,
        ReplacementEmailContent, ReplacementTemplate, Template,
    },
};

pub fn convert_email_entries(entries: Vec<BulkEamilEntriesSESV2>) -> Vec<BulkEmailEntry> {
    let bulk_eamil_entries: Vec<BulkEmailEntry> = entries
        .into_iter()
        .map(|bulk_email_entry| {
            BulkEmailEntry::builder()
                .destination(
                    Destination::builder()
                        .to_addresses(bulk_email_entry.to_email)
                        .build(),
                )
                .replacement_email_content(
                    ReplacementEmailContent::builder()
                        .replacement_template(
                            ReplacementTemplate::builder()
                                .replacement_template_data(
                                    bulk_email_entry.replacement_template_json,
                                )
                                .build(),
                        )
                        .build(),
                )
                .build()
        })
        .collect();
    return bulk_eamil_entries;
}

pub fn convert_email_content(content_subject: String, content_html: String) -> BulkEmailContent {
    let content: BulkEmailContent = BulkEmailContent::builder()
        .set_template(Some(
            Template::builder()
                .set_template_content(Some(
                    EmailTemplateContent::builder()
                        .subject(String::from(content_subject))
                        .html(String::from(content_html))
                        .build(),
                ))
                .template_data(String::from("{}"))
                .build(),
        ))
        .build();
    return content;
}

fn increment(map: &mut HashMap<String, u32>, status: String) {
    map.entry(status).and_modify(|val| *val += 1).or_insert(1);
}

pub async fn send_bulk_email(
    sesv2_client: aws_sdk_sesv2::Client,
    from_email: String,
    formatted_bulk_eamil_entries: Vec<BulkEmailEntry>,
    formatted_content: BulkEmailContent,
) -> Result<String, SendBulkEmailError> {
    // count time
    let begin_ts = time::SystemTime::now();
    let content_subject: String = formatted_content
        .template
        .as_ref()
        .unwrap()
        .template_content
        .as_ref()
        .unwrap()
        .subject
        .clone()
        .unwrap();

    let num_of_email = formatted_bulk_eamil_entries.len();
    let send_bulk_response_result = sesv2_client
        .send_bulk_email()
        .from_email_address(from_email.clone())
        .set_bulk_email_entries(Some(formatted_bulk_eamil_entries))
        .default_content(formatted_content)
        .send()
        .await;

    let mut counter: HashMap<String, u32> = HashMap::new();
    let mut err_log: String = String::from("");
    let result = match send_bulk_response_result {
        Ok(response) => {
            for res in response.bulk_email_entry_results {
                match res.status() {
                    Some(status) => increment(&mut counter, String::from(status.as_str())),
                    None => {}
                }
            }
            let success_num = counter.get(BulkEmailStatus::Success.as_str()).unwrap_or(&0);
            Ok(format!(
                "{success_num} email sent from {num_of_email} request."
            ))
        }
        // TODO: alert 추가 (필요시)
        Err(err) => {
            let service_error = err.into_service_error();
            err_log = service_error.to_string();
            Err(service_error)
        }
    };
    make_log_file_of_sesv2_response(from_email, content_subject, num_of_email, begin_ts, err_log);
    return result;
}
