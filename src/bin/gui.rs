use eframe::egui;
use timecalc::calc::calculate_input;
use timecalc::calc::seconds_from_string;
use timecalc::calc::string_from_seconds;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([159.0, 230.0])
            .with_resizable(false),
        ..Default::default()
    };
    let eframe_result = eframe::run_native(
        "TimeCalc",
        options,
        Box::new(|_cc| Ok(Box::<TimeCalc>::default())),
    );
    match eframe_result {
        Ok(_) => {
            println!();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

struct TimeCalc {
    display_text: String,
    result_string: String,
    seconds: i64,
    operator: String,
    input: String,
    exiting: bool,
}

impl Default for TimeCalc {
    fn default() -> Self {
        Self {
            display_text: "00:00:00".to_owned(),
            result_string: "00:00:00".to_owned(),
            seconds: 0,
            operator: "".to_owned(),
            input: "".to_owned(),
            exiting: false,
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
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
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
                        } else if *key == egui::Key::Q && i.modifiers.command {
                            self.exiting = true;
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
            if self.exiting {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            let total_height = row_height * 1.5 + ui.spacing().item_spacing.y;
            ui.add_sized(
                [144.0, total_height],
                egui::TextEdit::multiline(&mut self.display_text)
                    .desired_rows(2)
                    .font(egui::TextStyle::Monospace)
                    .interactive(false),
            );
            if ui
                .add_sized([144.0, total_height], egui::Button::new("AC"))
                .clicked()
            {
                clear(self);
            };
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
