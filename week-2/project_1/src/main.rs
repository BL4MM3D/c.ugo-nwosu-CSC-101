fn main() {
    let principal: f64 = 520_000_000.0; // Principal amount (P)
    let rate: f64 = 10.0; // Annual interest rate (R)
    let time: u32 = 5; // Time in years (n)

    // Compound Interest formula: A = P[1 + (R/100)]^n
    let amount = principal * (1.0 + rate / 100.0).powi(time as i32);

    // Calculate Compound Interest (CI = A - P)
    let compound_interest = amount - principal;

    println!("The compound interest for 5 years at 10% per annum is: {:.2}", compound_interest);
}
