use eframe::egui;
use std::io::{self, Write};

fn parse_int(time_string: &str) -> Option<i64> {
    match time_string.trim().parse::<i64>() {
        Ok(integer) => Some(integer),
        Err(_) => {
            println!("Could not parse string {}", time_string);
            None
        }
    }
}

fn seconds_from_string(time_string: String) -> Option<i64> {
    let colon_split: Vec<&str> = time_string.split(":").collect();
    if colon_split.len() == 3 {
        let hour: i64 = parse_int(colon_split[0])?;
        let minute: i64 = parse_int(colon_split[1])? + (hour * 60);
        let second: i64 = parse_int(colon_split[2])? + (minute * 60);
        Some(second)
    } else if colon_split.len() == 2 {
        let minute: i64 = parse_int(colon_split[0])?;
        let second: i64 = parse_int(colon_split[1])? + (minute * 60);
        Some(second)
    } else if colon_split.len() == 1 {
        let second: i64 = parse_int(colon_split[0])?;
        Some(second)
    } else {
        println!("Cannot parse seconds from string {}", time_string);
        None
    }
}

fn string_from_seconds(seconds: i64) -> String {
    let hour = seconds / 60 / 60;
    let padded_hour = format!("{:0>2}", hour);
    let minute = seconds / 60 % 60;
    let padded_minute = format!("{:0>2}", minute);
    let second = seconds % 60;
    let padded_second = format!("{:0>2}", second);
    format!("{0}:{1}:{2}", padded_hour, padded_minute, padded_second).to_string()
}

fn calculate_input(seconds: i64, input_string: String) -> Option<i64> {
    if input_string.trim().len() > 1 {
        if input_string.trim() == "exit" || input_string.trim() == "quit" {
            std::process::exit(0);
        }
        let operator = input_string.chars().next()?;
        let space = (input_string).chars().next()?;
        let slice: &str = if space == ' ' {
            &input_string[2..input_string.len()]
        } else {
            &input_string[1..input_string.len()]
        };
        let mut calculated_seconds: i64 = 0;
        let seconds_return = seconds_from_string(slice.to_string())?;
        if operator == '+' {
            calculated_seconds = seconds + seconds_return;
        } else if operator == '-' {
            calculated_seconds = seconds - seconds_return;
        } else if operator == '*' {
            calculated_seconds = seconds * seconds_return;
        } else if operator == '/' {
            calculated_seconds = seconds / seconds_return;
        }
        Some(calculated_seconds)
    } else {
        None
    }
}

fn main() -> eframe::Result {
    // let mut seconds: i64 = 0;
    // println!("Starting time: {}", string_from_seconds(seconds));
    //
    // loop {
    //     let mut input = String::new();
    //     print!("> ");
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut input).unwrap();
    //
    //     let calculated_seconds = calculate_input(seconds, input);
    //     match calculated_seconds {
    //         Some(s) => {
    //             seconds = s;
    //         }
    //         None => {
    //             println!("Could not calculate seconds from input");
    //         }
    //     };
    //     println!("{}", string_from_seconds(seconds));
    // }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([159.0, 184.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "TimeCalc",
        options,
        Box::new(|cc| Ok(Box::<TimeCalc>::default())),
    )
}

struct TimeCalc {
    display_text: String,
    result_string: String,
    seconds: u64,
    operator: String,
    input: String,
    operand: u64,
}

impl Default for TimeCalc {
    fn default() -> Self {
        Self {
            display_text: "00:00:00".to_owned(),
            result_string: "00:00:00".to_owned(),
            seconds: 0,
            operator: "".to_owned(),
            input: "".to_owned(),
            operand: 0,
        }
    }
}

fn render_display_text(state: &mut TimeCalc) {
    state.display_text = format!(
        "{}\n{} {}",
        state.result_string, state.operator, state.input
    );
}

impl eframe::App for TimeCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|i| {
                for event in &i.events {
                    if let egui::Event::Text(text) = event {
                        if text == "7" {
                            println!("pressed 7");
                        } else if text == "8" {
                            println!("pressed 8");
                        } else if text == "9" {
                            println!("pressed 9");
                        } else if text == "/" {
                            self.operator = "÷".to_owned();
                            render_display_text(self);
                            println!("pressed ÷");
                        } else if text == "4" {
                            println!("pressed 4");
                        } else if text == "5" {
                            println!("pressed 5");
                        } else if text == "6" {
                            println!("pressed 6");
                        } else if text == "x" || text == "*" {
                            self.operator = "x".to_owned();
                            render_display_text(self);
                            println!("pressed *");
                        } else if text == "1" {
                            println!("pressed 1");
                        } else if text == "2" {
                            println!("pressed 2");
                        } else if text == "3" {
                            println!("pressed 3");
                        } else if text == "-" {
                            self.operator = "-".to_owned();
                            render_display_text(self);
                            println!("pressed -");
                        } else if text == "0" {
                            println!("pressed 0");
                        } else if text == ":" || text == "." {
                            println!("pressed :");
                        } else if text == "=" {
                            println!("pressed =");
                        } else if text == "+" {
                            self.operator = "+".to_owned();
                            render_display_text(self);
                            println!("pressed +");
                        }
                    }
                }
            });
            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            let total_height = row_height * 2.0 + ui.spacing().item_spacing.y;
            ui.add_sized(
                [144.0, total_height],
                egui::TextEdit::multiline(&mut self.display_text)
                    .desired_rows(2)
                    .font(egui::TextStyle::Monospace)
                    .interactive(false),
            );
            egui::Grid::new("calc_grid")
                .striped(true)
                .min_col_width(30.0)
                .min_row_height(35.0)
                .show(ui, |ui| {
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("7"))
                        .clicked()
                    {
                        println!("Pressed 7");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("8"))
                        .clicked()
                    {
                        println!("Pressed 8");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("9"))
                        .clicked()
                    {
                        println!("Pressed 9");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("÷"))
                        .clicked()
                    {
                        println!("Pressed ÷");
                    };
                    ui.end_row();
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("4"))
                        .clicked()
                    {
                        println!("Pressed 4");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("5"))
                        .clicked()
                    {
                        println!("Pressed 5");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("6"))
                        .clicked()
                    {
                        println!("Pressed 6");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("x"))
                        .clicked()
                    {
                        println!("Pressed x");
                    };
                    ui.end_row();
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("1"))
                        .clicked()
                    {
                        println!("Pressed 1");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("2"))
                        .clicked()
                    {
                        println!("Pressed 2");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("3"))
                        .clicked()
                    {
                        println!("Pressed 3");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("-"))
                        .clicked()
                    {
                        println!("pressed -");
                    };
                    ui.end_row();
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("0"))
                        .clicked()
                    {
                        println!("pressed 0");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new(":"))
                        .clicked()
                    {
                        println!("pressed :");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("="))
                        .clicked()
                    {
                        println!("pressed =");
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("+"))
                        .clicked()
                    {
                        println!("pressed +");
                    };
                });
        });
    }
}
