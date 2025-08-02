pub fn search(array: &[i32], key: i32) -> Option<usize> {
    // let mut r = 0;
    // let l = array.len();

    Some(array.iter().rposition(|x| *x == key )?)
   /*  println!("{}", r);

    Some(0) */
}
