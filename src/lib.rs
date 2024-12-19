pub fn print_string(s: &str, to_uppercase: bool) -> String {
    if to_uppercase {
        s.to_uppercase()
    } else {
        s.to_string()
    }
}
