pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    let mut newKey = key;
    if newKey < 0{
        newKey = 26 + newKey;
    }
    for c in input.chars(){
        // println!("{}", c);
        if ('A'..='Z').contains(&c) {
            let x = ((c as u32 - 65 )+ newKey as u32) % 26 as u32 + 65;
            match char::from_u32(x){
                Some(c) => {
                    res.push(c);
                }
                None => (),
            }
            
        }else if ('a'..='z').contains(&c){
            let x = ((c as u32 - 97) + newKey as u32) % 26 as u32 + 97;
            match char::from_u32(x){
                Some(c) => {
                    res.push(c);
                }
                None => (),
            }
        }
            else{
            res.push(c);
        }
    } 

    res
}