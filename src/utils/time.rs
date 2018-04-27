use chrono::Local;
use chrono::DateTime;

pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn now_to_string() -> String {
    now().format("%F_%H-%M-%S_%f").to_string()
}