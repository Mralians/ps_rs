pub(super) fn get_status_value(status: &str, key: &str) -> Result<String, String> {
    let value = status
        .lines()
        .find(|line| matches!(line.find(key), Some(0)))
        .ok_or("Cannot find value".to_string())?
        .split_whitespace()
        .nth(1)
        .ok_or("Cannot get value".to_string())?;
    Ok(value.to_owned())
}
