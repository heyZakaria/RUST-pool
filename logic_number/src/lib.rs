


pub fn number_logic(num: u32) -> bool {
    let mut nbr = num;
    let mut hold = 0;
    let mut res = 0;
    let  c =num_of_digits(num);
    // println!("***{} {}", num, res);
     while nbr >= 1 {
        hold = nbr % 10;
        res += hold.pow(c);
        nbr /=  10;
    }
   res == num
}


fn num_of_digits(num: u32) -> u32{
    let mut c = 1;
    let mut n = num;
    while n > 9 {
        n = n / 10;
        c += 1;
        
    }

    c
}