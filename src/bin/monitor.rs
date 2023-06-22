//! Universal Retro Monitor
//!

use array_init::array_init;
use console::Term;

#[derive(Debug)]
struct VirtualMachine {
    ram: [u8; 65536],
}

fn main() {
    let mut vm = VirtualMachine {
        ram: array_init(|_| 0),
    };
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.write_line("Universal Retro Monitor v0.1").unwrap();
    term.write_str(":").unwrap();
    loop {
        match term.read_char() {
            Ok(ch) => match ch.to_ascii_uppercase() {
                'Q' => break,
                _ => continue,
            },
            Err(_) => unimplemented!("Unattended terminal error handler not implemented"),
        }
    }
    term.clear_screen().unwrap();
}
