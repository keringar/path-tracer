// Format seconds into a HH:MM:SS string
pub fn format_seconds(secs: u64) -> String {
    let hours = secs / 3600;
    let secs = secs % 3600;
    let minutes = secs / 60;
    let secs = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}