use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::time::SystemTime;

pub fn make_log_file_of_sesv2_response(
    from_email: String,
    content_subject: String,
    num_of_email: usize,
    begin_ts: SystemTime,
    error_log: String,
) -> io::Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("email_log")?;

    let latency = SystemTime::now()
        .duration_since(begin_ts)
        .unwrap()
        .as_millis();
    // Format: Number of Emails, Latency
    let log_entry = format!(
        "{},{},{},{},{}\n",
        from_email, content_subject, num_of_email, latency, error_log,
    );

    // Write to file
    log_file.write_all(log_entry.as_bytes())?;

    Ok(())
}
