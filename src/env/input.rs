use demand::{Confirm, Input};

use crate::app::theme;

pub async fn get_text_input(title: &str, placeholder: &str) -> String {
    let theme = theme();
    let input_text: String;
    loop {
        let t = Input::new(title)
            .placeholder(placeholder)
            .theme(&theme)
            .prompt("Input: ");

        let string = t.run().expect("error running input");

        if !string.trim().is_empty() {
            input_text = string;
            break;
        } else {
            println!("Invalid input. Please enter some text.");
            continue;
        }
    }
    input_text + ":1"
}

pub async fn confirmation() -> Result<bool, Box<dyn std::error::Error>> {
    let confirm = Confirm::new("Vanity Generation")
        .description("Select '1' for Starts-with and '2' for Ends-with")
        .affirmative("1")
        .negative("2")
        .selected(true)
        .run()
        .unwrap();

    Ok(confirm)
}
