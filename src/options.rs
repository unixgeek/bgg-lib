pub mod collections;
mod thing;

fn bool_to_param(name: &str, b: bool) -> String {
    if b {
        format!("&{name}=1")
    } else {
        String::default()
    }
}

fn u8_to_param(name: &str, value: Option<u8>) -> String {
    value
        .map(|v| format!("&{name}={}", v.to_owned()))
        .unwrap_or_default()
}

fn u16_to_param(name: &str, value: Option<u16>) -> String {
    value
        .map(|v| format!("&{name}={}", v.to_owned()))
        .unwrap_or_default()
}
