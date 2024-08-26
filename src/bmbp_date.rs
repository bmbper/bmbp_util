pub fn current_date() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d").to_string()
}
pub fn current_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_current_date() {
        println!("{}", current_date());
    }
    #[test]
    fn test_current_time() {
        println!("{}", current_time());
    }
}