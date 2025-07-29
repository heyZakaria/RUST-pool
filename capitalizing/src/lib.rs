pub fn capitalize_first(input: &str) -> String {
    let mut x = input.chars();

    match x.next() {
        Some(first_one_found) => {
            let mut result = first_one_found.to_uppercase().collect::<String>();
            result.push_str(x.as_str()); 
            result
        }
        None => "".to_string(),
    }
}


pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut new_word = true;

    for c in input.chars() {
        if c.is_whitespace() {
            new_word = true;
            result.push(c); 
        } else if new_word {
            result.extend(c.to_uppercase());
            new_word = false;
        } else {
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
    }

    result
}


 

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}
