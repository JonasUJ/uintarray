/*
 * Example that shows using a UintArray to pass data
 * using a single uint.
 *
 * This example is of course not very practical,
 * but it demonstrates the core principles of
 * the UintArray, which is its way of packing
 * multiple values into one.
 */

use uintarray::UintArray;

fn main() {
    let msg = "Hello, World!";
    println!("Message: {}", msg);

    // Convert the msg to a ua
    let ua = encode(msg);

    // ua.0 holds the actual u128 that stores the data
    println!("Encoded: {}", ua.0);

    // Collect the chars in the u128 (pretend we only had the u128 and not also a UintArray)
    let decoded_msg = decode(ua.0);
    println!("Decoded: {}", decoded_msg);
}

fn encode(msg: &str) -> UintArray {
    // New UintArray that stores elements the size of `u8`s
    let ua = UintArray::new::<u8>();

    // Add the chars to ua
    let ua = ua.extend(msg.as_bytes().into_iter().map(|c| *c as u128));

    ua
}

fn decode(uint: u128) -> String {
    // Get UintArray from a uint
    let ua = UintArray::from(uint);

    // Convert to strign
    ua.into_iter().map(|c| c as u8 as char).collect()
}
