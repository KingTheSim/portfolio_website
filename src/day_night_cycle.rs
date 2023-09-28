use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;
use regex::Regex;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Checks received date_time validity
const DATE_TIME_REGEX: &str = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d{3})?Z$";

#[wasm_bindgen]
extern "C" {
    fn getCurrentTime() -> Option<String>;
}

#[wasm_bindgen]
pub fn get_current_time() -> String {
    match { getCurrentTime() } {
        Some(current_time) => {
            if validate_date_time(&current_time) {
                return current_time;
            } else {
                log_error("Received an invalid date-time format");
            }
        },
        None => {
            log_error("Failed to get current time");
        }
    }

    "2023-09-28T12:00:00.000Z".to_string()
}

// Time validation function
fn validate_date_time(date_time: &str) -> bool {
    let regex = Regex::new(DATE_TIME_REGEX).unwrap();
    regex.is_match(date_time)
}

// Error logging function
fn log_error(error_message: &str) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    console::error_1(&JsValue::from_str(error_message));
}

fn extract_time(input: &str) -> u32 {
    let time_part = input.split('T').nth(1).unwrap();
    let hour_part = time_part.split(':').next().unwrap();
    hour_part.parse::<u32>().unwrap()
}

pub struct Background {
    is_day: bool,
}

pub enum Msg {
    ToggleDayNight(bool),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_day: bool,
}

impl Component for Background {
    type Message = Msg;
    type Properties = Props;

    fn create(props: §Self::Properties) -> Self {
        let is_day = extract_time(&get_current_time()) >= 7 && extract_time(&get_current_time()) < 19;
        let component = Background {
                is_day: props.is_day,
            };
        component
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDayNight(is_day) => {
                self.is_day = is_day;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let background_color = if self.is_day {
            "#87CEEB" // Daytime colour (light blue)
        } else {
            "#191970" // Nighttime colour (midnight blue)
        };

        html! {
            <div class="background" style=format!("background-color: {}", background_color)>
            </div>
        }
    }
}