use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut x = HashSet::new();
    let s1 = s.to_lowercase();

    let g = '京'.is_alphabetic();
    println!("****{}", g);
    for c in s1.chars() {
        if ('a'..='z').contains(&c) {
           
            x.insert(c);
        }
    }
   if x.len() == 26{
    return true;
   }
   false
}