fn main() {
    let max_number_u8: u8 = 255;
    let max_number_u16: u16 = 65535;
    let max_number_i8: i8 = 127;
    println!("The max number is {}", max_number_u8);
    println!("The max number is {}", max_number_u16);
    println!("The max number is {}", max_number_i8);

    let float_with_separator = 11_000.555_001;
    println!("float value {}", float_with_separator);

    let int_with_separator = 50_000;
    println!("int value {}", int_with_separator);
}