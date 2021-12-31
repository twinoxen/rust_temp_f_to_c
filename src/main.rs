fn main() {
    println!("32 in fahrenheit is {} in celsius", convert_fahrenheit_to_celsius(32));
    println!("72 in fahrenheit is {} in celsius", convert_fahrenheit_to_celsius(72));
    println!("110 in fahrenheit is {} in celsius", convert_fahrenheit_to_celsius(110));
}

fn convert_fahrenheit_to_celsius(num: i32) -> i32 {
    // Formula (32°F − 32) × 5/9 = 0°C

    (num - 32) * 5 / 9
}
