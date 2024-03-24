use std::io::{self, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn read_single_key(ctrlc_key: bool) -> io::Result<Key> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;

    let mut stdout = stdout.lock();
    write!(stdout, "{}", termion::cursor::Hide)?;

    let key = read_single_key_impl(&mut stdin.lock())?;

    write!(
        stdout,
        "{}{}",
        termion::cursor::Show,
        termion::cursor::BlinkingBlock
    )?;

    Ok(key)
}

fn read_single_key_impl<R: Read>(reader: &mut R) -> io::Result<Key> {
    let mut keys = Vec::new();
    for c in reader.keys() {
        let key = c?; // Handle the error from reader.keys()
        match key {
            Key::Ctrl('c') => break,
            Key::F(1) => {
                // Call a custom function or return a custom value for F1
                your_custom_f1_function();
                keys.push(Key::Null);
                break;
            }
            Key::F(2) => {
                // Call a custom function or return a custom value for F2
                your_custom_f2_function();
                keys.push(Key::Null);
                break;
            }
            Key::Char('1') => {
                // Call a custom function or return a custom value for numpad 1
                your_custom_numpad_1_function();
                keys.push(Key::Null);
                break;
            }
            Key::Char('2') => {
                // Call a custom function or return a custom value for numpad 2
                your_custom_numpad_2_function();
                keys.push(Key::Null);
                break;
            }
            // Add more cases for numpad 3 to 9 here
            Key::Char('9') => {
                // Call a custom function or return a custom value for numpad 9
                your_custom_numpad_9_function();
                keys.push(Key::Null);
                break;
            }
            key => keys.push(key),
        }
    }
    Ok(keys.first().cloned().unwrap_or(Key::Null))
}

fn your_custom_f1_function() {
    // Implement your custom logic for F1 here
    println!("F1 key pressed!");
}

fn your_custom_f2_function() {
    // Implement your custom logic for F2 here
    println!("F2 key pressed!");
}

fn your_custom_numpad_1_function() {
    // Implement your custom logic for numpad 1 here
    println!("Numpad 1 key pressed!");
}

fn your_custom_numpad_2_function() {
    // Implement your custom logic for numpad 2 here
    println!("Numpad 2 key pressed!");
}

fn your_custom_numpad_9_function() {
    // Implement your custom logic for numpad 9 here
    println!("Numpad 9 key pressed!");
}
