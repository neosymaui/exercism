pub fn reply(message: &str) -> &str {
    let m = message.trim_end();
    let is_y = m.contains(char::is_alphabetic) && m.to_uppercase() == m;
    let is_q = m.ends_with("?");

    match m {
        _m if m.is_empty() => "Fine. Be that way!",
        _m if is_y && is_q => "Calm down, I know what I'm doing!",
        _m if is_y => "Whoa, chill out!",
        _m if is_q => "Sure.",
        _ => "Whatever."
    }
}
