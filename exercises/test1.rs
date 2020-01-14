// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
fn calculate_apple_price(nb_apples: u32) -> u32 {
    let cutoff_nb = 40;
    let full_price = 2;
    let discount_price = 1;
    match nb_apples > cutoff_nb {
        true => nb_apples * discount_price,
        false => nb_apples * full_price,
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
