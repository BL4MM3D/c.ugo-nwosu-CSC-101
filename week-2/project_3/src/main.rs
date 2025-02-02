fn main() {
    let initial_value: f64 = 510_000.0; // Initial value of the TV (P)
    let depreciation_rate: f64 = 5.0; // Annual depreciation rate (R)
    let time: u32 = 3; // Time in years (n)

    // Depreciation formula: A = P[1 - (R/100)]^n
    let depreciated_value = initial_value * (1.0 - depreciation_rate / 100.0).powi(time as i32);

    println!("The value of the TV after 3 years of depreciation is: {:.2}", depreciated_value);
}
