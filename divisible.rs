// Import the necessary crates
use num_bigint::BigUint; // For handling large unsigned integers
use num_integer::Integer; // For integer division and remainder operations

fn main(){
    // Define the large number and the divisor as strings
    let large_number = "218383819917119";
    let divisor = "12886258826";
    
    // Parse the strings into BigUint (Big Unsigned Integer) types
    // `parse_bytes` takes a byte slice and a radix (base), here 10 for decimal
    let num = BigUint::parse_bytes(large_number.as_bytes(), 10).unwrap();
    let div = BigUint::parse_bytes(divisor.as_bytes(), 10).unwrap();
    
    // Perform integer division and get both quotient and remainder
    let (quotient, remainder) = num.div_rem(&div);
    
    // Print the quotient and remainder
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}
