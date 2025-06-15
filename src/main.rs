use std::io;

fn main() {
    println!("Hello World!");
    add_2_numbers(5, 4);
    println!("Please enter your weight");
    let mut _your_wght = String::new();

    io::stdin()
        .read_line(&mut _your_wght)
        .expect("Failed to read your weight");
    println!("Please enter your height");

    let _your_wght: f32 = _your_wght
        .trim()
        .parse()
        .expect("Please enter weight in decimal value");

    let mut _your_hght = String::new();

    io::stdin()
        .read_line(&mut _your_hght)
        .expect("Failed to read your height");

    let _your_hght: f32 = _your_hght
        .trim()
        .parse()
        .expect("Please enter your height in decimal values");

    calc_bmi(_your_wght, _your_hght);
}

fn add_2_numbers(a: i8, b: i8) {
    let add = a + b;
    println!("The sum of the two numbers is: {add}");
}

fn calc_bmi(wght: f32, hght: f32) {
    let total_bmi: f32 = (wght) / (hght * hght);
    println!("Your total BMI is : {total_bmi}")
}
