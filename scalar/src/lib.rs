pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16{
    a - b
}


pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}


pub fn quo(a: f32, b: f32) -> f32{
    a / b
}


pub fn rem(a: f32, b: f32) -> f32{
    a % b
}



fn main() {
    // sum
    println!("sum: {}", sum(234, 2)); // 'sum: 236'
    println!("sum: {}", sum(1, 255)); // 'ERROR: attempt to add with overflow'

    // diff
    println!("diff: {}", diff(234, 2)); // 'diff: 232'
    println!("diff: {}", diff(-32768, 32766)); // 'ERROR: attempt to subtract with overflow'

    // product
    println!("pro: {}", pro(23, 2)); // 'pro: 46'
    println!("pro: {}", pro(-128, 2)); // 'ERROR: attempt to multiply with overflow'

    // quotient
    println!("quo: {}", quo(22.0, 2.0)); // 'quo: 11'
    println!("quo: {}", quo(-128.23, 2.0)); // 'quo: -64.115'

    // remainder
    println!("rem: {}", rem(22.0, 2.0)); // 'rem: 0'
    println!("rem: {}", rem(-128.23, 2.0)); // 'rem: -0.22999573'
}