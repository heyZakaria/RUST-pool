pub fn rev_str(s: &str) -> String {
    let mut res = String::new();

    for c in s.chars().rev(){
        res.push(c) 
    }
    println!("{}", res);
    return res;
}