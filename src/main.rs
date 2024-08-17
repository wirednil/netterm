mod lexer_form;
mod customs;
mod form;
use log::{info};

use crate::form::Form;

fn main() {
    info!("Inicio del programa.");
    let fm_result: Result<Form, std::io::Error> = Form::new("clie.form");
    match fm_result {
        Ok(mut fm) => {
            println!("Form loaded successfully.");
            fm.make_form();
        },
        Err(e) => {
            eprintln!("Error loading form: {}", e);
        },
    }
}
