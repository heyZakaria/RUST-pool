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

