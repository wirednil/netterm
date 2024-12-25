use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs::File;
use std::io::{Read, self};
use crate::config::configurable_form::FieldType;
use crate::config::configurable_form::ConfigurableForm;

#[derive(Debug, Deserialize)]
struct InputField {
    id: String,
    label: String,
    #[serde(rename = "type")]
    input_type: String,
    size: Option<usize>, // Para campos de texto o numérico
    format: Option<String>, // Para campos con formato como fecha o decimales
}

#[derive(Debug, Deserialize)]
pub struct Form {
    title: String,
    input: Vec<InputField>,
}

fn limit_length(value: usize) -> usize {
    if value > 30 {
        30
    } else {
        let fix_val = value+1;
        fix_val
    }
}

impl Form {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let mut file = File::open(file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let form: Form = from_str(&buffer).map_err(|err| {
            io::Error::new(io::ErrorKind::InvalidData, format!("XML Parsing Error: {:?}", err))
        })?;

        Ok(form)
    }

    pub fn make_form(&mut self) {
        println!("Formulario: {}", self.title);
        let mut form = ConfigurableForm::new();
        for field in &self.input {
            form.add_label(&field.label);
            match field.input_type.as_str() {
                "text" => {
                    form.add_field(field.id.clone(), FieldType::Text, limit_length(field.size.unwrap()));
                    // println!("Campo de texto: {} (Size: {:?})", field.label, field.size);
                }
                "numeric" => {
                    form.add_field(field.id.clone(), FieldType::Number, limit_length(field.size.unwrap()));
                    println!("Campo numérico: {} (Size: {:?})", field.label, field.size);
                }
                "decimal" => {
                    form.add_field(field.id.clone(), FieldType::Number, limit_length(field.size.unwrap()));
                    if let Some(format) = &field.format {
                        println!("Campo decimal: {} (Format: {})", field.label, format);
                    }
                }
                "date" => {
                    form.add_field(field.id.clone(), FieldType::Date, limit_length(field.format.clone().unwrap().len()));
                    if let Some(format) = &field.format {
                        println!("Campo de fecha: {} (Format: {})", field.label, format);
                    }
                }
                "time" =>  {
                    form.add_field(field.id.clone(), FieldType::Time, limit_length(field.format.clone().unwrap().len()));
                    if let Some(format) = &field.format {
                        println!("Campo de time: {} (Format: {})", field.label, format);
                    }
                }
                _ => {
                    println!("Campo no soportado: {}", field.label);
                }
            }
        }
        form.show();
    }
}
