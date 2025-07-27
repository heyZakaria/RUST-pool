pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let mut check = false;
    if c < 0 {
        check = true;
        
    }
    let mut x: f64 = c.to_owned().into();
    let e:f64 = x.exp();
    if x < 0.0{
        x *= -1.0
    }
    let mut l: f64 = x.ln();
    // println!("#######{}", l);
   
    (c, e,  l  )
}

pub fn str_function(a: String)  -> (String, String)  {
     //let a = "1 2 4 5 6".to_owned();
    let x:Vec<char> = a.chars().collect();
    let mut res = String::new();
    for ( i , v) in x.iter().enumerate() {
        if *v == ' '{
            continue
        }
        let num:f64 = v.to_string().parse().unwrap_or(123.0);
        // num.to_string();
        res.push_str(&num.exp().to_string());
        if i != x.len() -1 {

            res.push_str(" ");
        }
        // println!("***{}*** {}", num.ln().to_string(), num);
    }

    (a, res)
}


pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
   let mut res: Vec<f64> = vec![];
    for  v in &b {
        let x: f64 = v.to_owned().into();
        let e:f64 = x.ln();
        // println!("***{}", e);
            res.push(e);
    }
    (b,res )
}