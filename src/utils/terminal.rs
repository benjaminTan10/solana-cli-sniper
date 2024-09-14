pub fn clear_screen() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
