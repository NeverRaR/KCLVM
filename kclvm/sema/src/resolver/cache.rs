pub fn test_rust_bug(number: f32) -> f32 {
    let x2 = number * 0.5;
    let y = number;
    let threehalfs = 1.5_f32;

    let i = unsafe { *(&y as *const f32 as *const i32) }; // evil floating point bit level hacking
    let i = 0x5f3759df - (i >> 1);
    let y = unsafe { *(&i as *const i32 as *const f32) };
    let y = y * (threehalfs - (x2 * y * y)); // 1st iteration
    let y = y * (threehalfs - (x2 * y * y)); // 2nd iteration, this can be removed
    return y;
}

#[test]
fn test() {
    let input = 1000.0;
    let a = test_rust_bug(input);
    println!("{}", 1.0 / input.sqrt() - a);
}
