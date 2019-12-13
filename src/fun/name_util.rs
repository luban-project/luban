use std::char;

pub fn is_valid_identifier_char(ch: &char) -> bool {
    if ch.is_alphanumeric() {
        return true;
    }
    if ch.eq(&'-') {
        return true;
    }
    if ch.eq(&'_') {
        return true;
    }
    return false;
}

pub fn is_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if !s.chars().all(|c| is_valid_identifier_char(&c)) {
        return false;
    };
    return s.chars().next().unwrap().is_alphabetic();
}

pub fn is_upper_snake_case(s: &str) -> bool {

}

pub fn is_lower_snake_case(s: &str) -> bool {

}



pub fn is_upper_hyphen_case(s: &str) -> bool {
    return s.contains("-");
}

pub fn is_lower_hyphen_case(s: &str) -> bool {
    return s.contains("-");
}

pub fn is_lower_case(s: &str) -> bool {
    return s.chars().all(|c| c.is_lowercase());
}

pub fn is_upper_case(s: &str) -> bool {
    return s.chars().all(|c| c.is_uppercase());
}

pub fn is_camel_case(s: &str) -> bool {
    return true;
}

pub fn is_upper_camel_case(s: &str) -> bool {
    return true;
}

pub fn is_lower_camel_case(s: &str) -> bool {
    return true;
}


