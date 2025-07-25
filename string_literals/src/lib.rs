pub fn is_empty(s: &str) -> bool {
    /* if s.chars().count() > 0{
        return true;
    }
    false */

    s.is_empty()
}

pub fn is_ascii(v: &str) -> bool{
    // let hold = s.chars().collect();

   /*  for c in v.chars() {
        if ((c as u8) < 1) || ((c as u8) > 127) {
            return false;
        }
    }

    true */

    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let (p1, p2) = v.split_at(index);

    (p1, p2)
    
}

pub fn find(v: &str, pat: char) -> usize{
   // let x = v.rfind(pat);


    for (i, v) in v.chars().enumerate(){
      //  println!("***************{}", v);
        if v == pat {
            return i;
        }
    } 
    0
}