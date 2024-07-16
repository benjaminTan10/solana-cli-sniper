use std::error::Error;

use console::Term;

pub async fn read_keys() -> Result<(), Box<dyn Error + Send>> {
    println!("Press any key to exit...");

    let term = Term::stdout();

    loop {
        match term.read_key().unwrap() {
            _ => {
                // Break the loop when any key is pressed
                break;
            }
        }
    }

    Ok(())
}
