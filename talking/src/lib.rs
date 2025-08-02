pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let so2al = text.trim_end().ends_with('?');
    let letters: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let wa33 = !letters.is_empty() && letters.iter().all(|c| c.is_uppercase());

    if wa33 && so2al {
        return "Quiet, I am thinking!";
    } else if wa33 {
        return "There is no need to yell, calm down!";
    } else if so2al {
        return "Sure.";
    } else {
        return "Interesting";
    }
}
