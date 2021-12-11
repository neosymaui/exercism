pub fn abbreviate(phrase: &str) -> String {
    let mut res = String::new();

    for word in phrase.split(|c: char| !c.is_alphabetic()) {
        if word.len() > 0 {
            if word.chars().all(char::is_uppercase) {
                res.push_str(&word.chars().next().unwrap().to_string());
            } else if word.chars().all(char::is_lowercase) {
                res.push_str(
                    &word
                        .chars()
                        .next()
                        .unwrap()
                        .to_uppercase()
                        .collect::<String>(),
                );
            } else {
                res.push_str(
                    &word
                        .chars()
                        .filter(|&c| c.is_uppercase())
                        .collect::<String>(),
                );
            }
        }
    }
    res
}
