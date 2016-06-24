pub fn reply(question: &str) -> String {
    if question.chars().last() == Some('?') {
        // questions
        "Sure.".to_string()
    } else if question == "" {
        "Fine. Be that way!".to_string()
    } else if question.chars().all(|x| !x.is_alphabetic() || x.is_uppercase()) {
        // (alphabetic => uppercase) => yelling
        "Whoa, chill out!".to_string()
    } else {
        "Whatever.".to_string()
    }
}
