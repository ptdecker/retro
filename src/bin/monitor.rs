use console::Term;
fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.write_line("Universal Retro Monitor v0.1").unwrap();
    term.write_str(":").unwrap();
    loop {
        match term.read_char() {
            Ok(ch) if ch == 'Q' => break,
            Err(_) => unimplemented!("Unattended terminal error handler not implemented"),
            _ => (),
        }
    }
    term.clear_screen().unwrap();
}
