use std::io::{self, Write};

fn parse_int(time_string: &str) -> Result<i64, std::num::ParseIntError> {
    let integer: i64 = time_string.trim().parse()?;
    println!("integer in parse_int: {}", integer);
    Ok(integer)
}

fn from_string(time_string: String) -> Result<i64, std::num::ParseIntError> {
    let colon_split: Vec<&str> = time_string.split(":").collect();
    if colon_split.len() == 3 {
        let hour: i64 = parse_int(colon_split[0])?;
        let minute: i64 = parse_int(colon_split[1])? + (hour * 60);
        let second: i64 = parse_int(colon_split[2])? + (minute * 60);
        println!("hour: {}", hour);
        println!("minute: {}", minute);
        println!("second: {}", second);
        Ok(second)
    } else if colon_split.len() == 2 {
        let minute: i64 = parse_int(colon_split[0])?;
        let second: i64 = parse_int(colon_split[1])? + (minute * 60);
        println!("minute: {}", minute);
        println!("second: {}", second);
        Ok(second)
    } else if colon_split.len() == 1 {
        let second: i64 = parse_int(colon_split[0])?;
        println!("second: {}", second);
        Ok(second)
    } else {
        "".parse::<i64>()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    struct ClockTime {
        second: i64,
        minute: i64,
        hour: i64,
    }
    let clock_time = ClockTime {
        second: 0,
        minute: 0,
        hour: 0,
    };
    println!(
        "Starting time: {:02}:{:02}:{:02}",
        clock_time.hour, clock_time.minute, clock_time.second
    );
    let mut input = String::new();
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let seconds_result = from_string(input);
    match seconds_result {
        Ok(seconds) => {
            println!("Seconds: {}", seconds);
        }
        Err(e) => {
            eprintln!(
                "Error parsing time entry.  Should be in the format HH:MM:SS, MM:SS, or SS: {}",
                e
            );
        }
    }

    Ok(())
}
