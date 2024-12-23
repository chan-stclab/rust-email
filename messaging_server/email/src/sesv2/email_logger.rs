use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::time::SystemTime;

pub fn make_log_file_of_sesv2_response(num_email: usize, start_at: SystemTime) -> io::Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("email_log")?;

    let latency = SystemTime::now()
        .duration_since(start_at)
        .unwrap()
        .as_millis();
    // Format: Number of Emails, Latency
    let log_entry = format!("{},{}\n", num_email, latency);

    // Write to file
    log_file.write_all(log_entry.as_bytes())?;

    Ok(())
}
