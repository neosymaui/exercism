pub fn is_opening(c: char) -> bool {
    match c {
        '{' | '(' | '[' => true,
        _ => false,
    }
}

pub fn is_closing(c: char) -> bool {
    match c {
        '}' | ')' | ']' => true,
        _ => false,
    }
}

pub fn get_closure(c: char) -> char {
    match c {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        _ => '?',
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut parsing = vec![];

    for c in string.chars() {
        if is_opening(c) {
            parsing.push(c);
        }

        if is_closing(c) {
            match parsing.last() {
                Some(d) => {
                    if get_closure(*d) == c {
                        parsing.pop();
                    } else {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }
    parsing.is_empty()
}
