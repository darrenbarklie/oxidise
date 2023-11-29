fn main() {
    println!("Hello, world!");
    
    let result = calc_decimal_values();
    println!("{}", result);
}


use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn calc_decimal_values () -> Decimal {
    // let result = 42;
    // let result = dec!(42) - dec!(12.50);
    // let result = dec!(0.01) + dec!(0.02);
    
    //let amount = dec!(25.42);
    //let tax_percentage = dec!(1.2);
    //let result = amount + (amount * tax_percentage).round_dp(2);

    //assert_eq!(result, dec!(55.92));



    let amount = dec!(99.99);
    let tax_rate = dec!(0.2);
    let tax_calc = (amount / tax_rate).round_dp(2);
    
    let result = tax_calc;


    result
}
