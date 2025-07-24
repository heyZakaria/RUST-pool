pub fn fibonacci(n: u32) -> u32 {
   if n == 0 {
    return 0
   }
    if n == 1 {
    return 1
   }

    let mut f0 = 0;
    let mut f1 = 1;

    let mut res = 0;

    for _ in 1..n {
        res = f0 + f1;
        // println!("f0 --- {}  \nf1 ---- {} \nres ********* {}", f0, f1, res);
        f0 = f1;
        f1 = res;

    }
    return res;
}
