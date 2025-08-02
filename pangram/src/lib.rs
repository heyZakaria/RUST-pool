use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut x = HashSet::new();

    for c in s.chars() {
        if c.is_alphabetic(){
           let r = c.to_lowercase();
            x.insert(r);
        }
    }
   if x.len() == 26{
    return true;
   }
   false
}