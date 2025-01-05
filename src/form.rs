use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs::File;
use std::io::{Read, self};
use crate::config::configurable_form::FieldType;
use crate::config::configurable_form::ConfigurableForm;
use cursive::views::LinearLayout;

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
struct Container {
    #[serde(rename = "input")]
    inputs: Option<Vec<InputField>>,
    #[serde(rename = "vertical")]
    vertical: Option<Vec<Container>>,
    #[serde(rename = "horizontal")]
    horizontal: Option<Vec<Container>>,
}

#[derive(Debug, Deserialize)]
pub struct Form {
    title: String,
    #[serde(rename = "conteiner")]
    root_vertical: Option<Vec<Container>>,
}

fn limit_length(value: usize) -> usize {
    if value > 30 { 30 } else { value+1 }
}

impl Form {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let mut file   = File::open(file_name)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let form: Form = from_str(&buffer).map_err(|err| {
            io::Error::new(io::ErrorKind::InvalidData, format!("XML Parsing Error: {:?}", err))
        })?;

        Ok(form)
    }

    pub fn make_form(&mut self) {
        let mut form = ConfigurableForm::new();
        let mut vertical_root_layout = LinearLayout::vertical();
        println!("make form 1");
        if let Some(containers) = &self.root_vertical{
            println!("make form 2");
            for container in containers {
                println!("make form 3");
                self.process_container(container, &mut form, &mut vertical_root_layout); // false para vertical
            }
            println!("make form 4");
            form.show(&self.title, vertical_root_layout);
            
        println!("make form 5");
        }
        println!("make form 6");
    }

    fn process_container(&self, container: &Container, form: &mut ConfigurableForm, layout: &mut LinearLayout) {
        
        println!("process_container");
        if let Some(inputs) = &container.inputs {
            for field in inputs {
                println!("adding label, input");
                form.add_label(&field.label);
                match field.input_type.as_str() {
                    "text" => {
                        form.add_field(field.id.clone(), 
                                       FieldType::Text, 
                                       limit_length(field.size.unwrap()));
                        // println!("Campo de texto: {} (Size: {:?})", field.label, field.size);
                    }
                    "numeric" => {
                        form.add_field(field.id.clone(),
                                       FieldType::Number, 
                                       limit_length(field.size.unwrap()));
                        // println!("Campo numérico: {} (Size: {:?})", field.label, field.size);
                    }
                    "decimal" => {
                        form.add_field(field.id.clone(), 
                                       FieldType::Number, 
                                       limit_length(field.size.unwrap()));
                        if let Some(format) = &field.format {
                            // println!("Campo decimal: {} (Format: {})", field.label, format);
                        }
                    }
                    "date" => {
                        form.add_field(field.id.clone(), 
                                       FieldType::Date, 
                                       limit_length(field.format.clone().unwrap().len()));
                        if let Some(format) = &field.format {
                            println!("Campo de fecha: {} (Format: {})", field.label, format);
                        }
                    }
                    "time" =>  {
                        form.add_field(field.id.clone(), 
                                       FieldType::Time, 
                                       limit_length(field.format.clone().unwrap().len()));
                        if let Some(format) = &field.format {
                            println!("Campo de time: {} (Format: {})", field.label, format);
                        }
                    }
                    _ => {
                        println!("Campo no soportado: {}", field.label);
                    }
                }
            }
            form.push_fields_layout(layout);
            form.clear_cache();
        }


        if let Some(verticals) = &container.vertical {
            for v in verticals {
                let mut vertical = LinearLayout::vertical();
                self.process_container(v, form, &mut vertical);
                layout.add_child(vertical);
            }
        }

        if let Some(horizontals) = &container.horizontal {
            for h in horizontals {
                let mut horizontal = LinearLayout::horizontal();
                self.process_container(h, form, &mut horizontal);
                layout.add_child(horizontal);
            }
        }
    }
}
