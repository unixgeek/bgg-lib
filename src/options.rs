mod thing;

fn bool_to_string(b: bool) -> String {
    if b {
        "1".to_owned()
    } else {
        "0".to_owned()
    }
}