use std::env;
mod word_box;

fn main() {
    let mut input: Vec<String> = env::args().collect();

    input.remove(0);

    let input_str: String = input.join("");

    let input_str_cleaned: String = input_str.chars().filter(|c| c.to_owned() != ' ').collect();

    println!(
        "input: {}, length: {}",
        input_str.to_uppercase(),
        input_str_cleaned.len()
    );

    match word_box::pack_box(input_str_cleaned.to_uppercase()) {
        Result::Ok(output) => {
            let (x, y) = output.dimensions;
            println!("dimensions: ({}, {})", x, y);
            println!("{}", output);
        }
        Result::Err(err_msg) => {
            println!("wordbox: error: {}", err_msg);
            std::process::exit(1);
        }
    }
}
