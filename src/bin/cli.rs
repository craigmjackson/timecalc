use std::io::Write;
use timecalc::calc::calculate_input;
use timecalc::calc::string_from_seconds;

fn main() {
    let mut seconds: i64 = 0;
    println!("Starting time: {}", string_from_seconds(seconds));
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        let calculated_seconds = calculate_input(seconds, input);
        match calculated_seconds {
            Some(s) => {
                seconds = s;
            }
            None => {
                println!("Could not calculate seconds from input");
            }
        };
        println!("{}", string_from_seconds(seconds));
    }
}
