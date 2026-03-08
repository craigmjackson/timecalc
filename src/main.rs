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
            .with_inner_size([159.0, 200.0])
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
    seconds: i64,
    operator: String,
    input: String,
    operand: i64,
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

fn clear(state: &mut TimeCalc) {
    state.result_string = "00:00:00".to_owned();
    state.operator = "".to_owned();
    state.input = "".to_owned();
    render_display_text(state);
}

fn add_input_character(state: &mut TimeCalc, character: String) {
    let colon_split: Vec<&str> = state.input.split(":").collect();
    let num_elements = colon_split.len();
    if character == ":" {
        if num_elements == 1 {
            let num_characters = colon_split[0].chars().count();
            if num_characters == 0 {
                return;
            }
        } else if num_elements == 2 {
            let num_characters = colon_split[1].chars().count();
            if num_characters == 0 {
                return;
            }
        } else if num_elements == 3 {
            return;
        }
    } else if num_elements == 1 {
        let num_characters = colon_split[0].chars().count();
        if num_characters > 1 {
            return;
        }
    } else if num_elements == 2 {
        let num_characters = colon_split[1].chars().count();
        if num_characters > 1 {
            return;
        }
    } else if num_elements == 3 {
        let num_characters = colon_split[2].chars().count();
        if num_characters > 1 {
            return;
        }
    }
    state.input = format!("{}{}", state.input, character);
}

fn calculate_from_gui_input(state: &mut TimeCalc) {
    let new_input = state.input.clone();
    if state.operator.is_empty() {
        let calculated_seconds = seconds_from_string(new_input);
        match calculated_seconds {
            Some(s) => {
                state.seconds = s;
                state.result_string = string_from_seconds(state.seconds);
                state.input = "".to_owned();
                state.operator = "".to_owned();
                render_display_text(state);
                return;
            }
            None => {
                println!("Could not calculate seconds from input");
                return;
            }
        }
    }
    if state.operator == "÷" {
        state.operator = "/".to_owned();
    }
    let input_with_operator = format!("{} {}", state.operator, new_input);
    let calculated_seconds = calculate_input(state.seconds, input_with_operator);
    match calculated_seconds {
        Some(s) => {
            state.seconds = s;
            state.result_string = string_from_seconds(state.seconds);
            state.input = "".to_owned();
            state.operator = "".to_owned();
            render_display_text(state);
        }
        None => {
            println!("Could not calculate seconds from input");
        }
    };
}

impl eframe::App for TimeCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key, pressed: true, ..
                    } = event
                    {
                        if *key == egui::Key::Enter {
                            calculate_from_gui_input(self);
                        } else if *key == egui::Key::Escape {
                            clear(self);
                        }
                    }
                    if let egui::Event::Text(text) = event {
                        if text == "7" {
                            add_input_character(self, "7".to_owned());
                            render_display_text(self);
                        } else if text == "8" {
                            add_input_character(self, "8".to_owned());
                            render_display_text(self);
                        } else if text == "9" {
                            add_input_character(self, "9".to_owned());
                            render_display_text(self);
                        } else if text == "/" {
                            self.operator = "÷".to_owned();
                            render_display_text(self);
                        } else if text == "4" {
                            add_input_character(self, "4".to_owned());
                            render_display_text(self);
                        } else if text == "5" {
                            add_input_character(self, "5".to_owned());
                            render_display_text(self);
                        } else if text == "6" {
                            add_input_character(self, "6".to_owned());
                            render_display_text(self);
                        } else if text == "x" || text == "*" {
                            self.operator = "x".to_owned();
                            render_display_text(self);
                        } else if text == "1" {
                            add_input_character(self, "1".to_owned());
                            render_display_text(self);
                        } else if text == "2" {
                            add_input_character(self, "2".to_owned());
                            render_display_text(self);
                        } else if text == "3" {
                            add_input_character(self, "3".to_owned());
                            render_display_text(self);
                        } else if text == "-" {
                            self.operator = "-".to_owned();
                            render_display_text(self);
                        } else if text == "0" {
                            add_input_character(self, "0".to_owned());
                            render_display_text(self);
                        } else if text == ":" || text == "." {
                            add_input_character(self, ":".to_owned());
                            render_display_text(self);
                        } else if text == "+" {
                            self.operator = "+".to_owned();
                            render_display_text(self);
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
                        add_input_character(self, "7".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("8"))
                        .clicked()
                    {
                        add_input_character(self, "8".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("9"))
                        .clicked()
                    {
                        add_input_character(self, "9".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("÷"))
                        .clicked()
                    {
                        self.operator = "÷".to_owned();
                        render_display_text(self);
                    };
                    ui.end_row();
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("4"))
                        .clicked()
                    {
                        add_input_character(self, "4".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("5"))
                        .clicked()
                    {
                        add_input_character(self, "5".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("6"))
                        .clicked()
                    {
                        add_input_character(self, "6".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("x"))
                        .clicked()
                    {
                        self.operator = "x".to_owned();
                        render_display_text(self);
                    };
                    ui.end_row();
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("1"))
                        .clicked()
                    {
                        add_input_character(self, "1".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("2"))
                        .clicked()
                    {
                        add_input_character(self, "2".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("3"))
                        .clicked()
                    {
                        add_input_character(self, "3".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("-"))
                        .clicked()
                    {
                        self.operator = "-".to_owned();
                        render_display_text(self);
                    };
                    ui.end_row();
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("0"))
                        .clicked()
                    {
                        add_input_character(self, "0".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new(":"))
                        .clicked()
                    {
                        add_input_character(self, ":".to_owned());
                        render_display_text(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("="))
                        .clicked()
                    {
                        calculate_from_gui_input(self);
                    };
                    if ui
                        .add_sized([ui.available_width(), 30.0], egui::Button::new("+"))
                        .clicked()
                    {
                        self.operator = "+".to_owned();
                        render_display_text(self);
                    };
                });
        });
    }
}
