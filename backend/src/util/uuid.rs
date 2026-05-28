use uuid::Uuid;

pub fn parse_uuid(value: &str) -> Option<String> {
    Uuid::parse_str(value.trim())
        .ok()
        .map(|u| u.to_string())
}

pub fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}
