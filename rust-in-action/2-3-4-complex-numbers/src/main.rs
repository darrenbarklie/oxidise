use num::complex::Complex;                      //<1>

fn main() {
  let a = Complex { re: 2.1, im: -1.2 };        //<2>
  let b = Complex::new(11.1, 22.2);             //<3>
  let result = a + b;

  println!("{} + {}𝑖", result.re, result.im)    //<4>
}

// 1. The use keyword brings the Complex type into local scope
// 2. Every Rust type has a literal syntax
// 3. Most types implement a new() static method
// 4. Accesses fields with the dot operator
