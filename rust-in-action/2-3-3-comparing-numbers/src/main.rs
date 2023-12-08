use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    let c: u16 = 1000;

    // promotion: safest to cast smaller type to larger one
    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }

    let c_ = c.try_into().unwrap();

    if a < c_ {
        println!("Ten is less than one thousand.");
    }

    // error: assertion failed
    // assert!(0.1 + 0.2 == 0.3);
   
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!();

    
    println!("abc (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // expect error: assertation failed
    // assert!(xyz.0 + xyz.1 == xyz.2);

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);

    // expect_error: assertion `left == right` failed
    // let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);
    
    let y: f32 = 1.0 / 0.0;
    assert!(y.is_finite());

    println!("Done!");
}
