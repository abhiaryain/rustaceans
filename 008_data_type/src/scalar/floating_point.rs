// ======================================================================================
//                             RUST FLOATING-POINT REFERENCE
// ======================================================================================
//  Type  | Width   | Precision         | Min Value           | Max Value
// -------+---------+-------------------+---------------------+-------------------------
//  f32   | 32-bit  | ~7 decimal digits | -3.40282347e+38     | 3.40282347e+38
//  f64   | 64-bit  | ~15 decimal digits| -1.7976931348623157e+308 | 1.7976931348623157e+308
// ======================================================================================
// Note: Rust's default floating point type is `f64`, because modern CPUs process it at roughly the same speed as f32 while delivering substantially higher mathematical precision.
//       All floating-point types are signed.
// ======================================================================================

pub fn floating_point() {
    let weight = 10.5; // f64
    println!("weight is {}", weight);
}
