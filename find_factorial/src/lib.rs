pub fn factorial(num: u64) -> u64 {
    
    if num == 1 || num == 0{
        return 1;
    }
    let mut x = 1;
    for i in 1..num {
        x += x * i
    }

    return x;
}
