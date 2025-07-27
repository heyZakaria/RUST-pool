pub fn initials(names: Vec<&str>) -> Vec<String> 
{
    // let mut hold: Vec<&str> = vec![];
    let mut res = String::new();
    let mut fine: Vec<String> = vec![];

    /* let mut name = String::from("Soufiane");

    name.push('x');
    
    fine.push(name.as_str());
    name.push('x');
    
    println!("{}", name.as_str());
 */


     for (_, v) in names.into_iter().enumerate() {    
        for (x, y) in v.chars().enumerate(){
            if x == 0 {
                res.push(y);
                res.push('.');
                res.push(' ');
            }
            if y == ' '  {
                res.push(v.chars().nth(x+1).expect("sdsdsd"));
                res.push('.');
                // res.push(' ');

            }
            
        }
        if res.len() > 0{
                fine.push(res.clone());
                res.clear();

            }
    } 


    fine
}



