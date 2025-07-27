pub fn first_subword(mut s: String) -> String {
    
    let v:Vec<char> = s.chars().collect();
    let mut res = String::new();
    // println!("*****+++++++++++++++++{:?}", v);
    for (i, x) in v.iter().enumerate() {
        println!("------------{}", x);
        
        if ((*x as u8 >= 65 && *x as u8 <= 90) && i != 0) || *x == '_'      {
            // println!("+++++++++++++++++{}", x);
            break;
        }
        res.push(*x);
    }

    // println!("****************`{}", res);
    
    res
}


/* pub fn first_subword(s: String) -> String {
    let pos = s
        .char_indices()
        .position(|(i, c)| i != 0 && (c.is_ascii_uppercase() || c == '_'))
        .unwrap_or(s.len());

    s[0..pos].to_owned()
}
     */
