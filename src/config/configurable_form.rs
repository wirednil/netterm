use crate::customs::views::edit_date_view::EditDateView;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use cursive::CursiveExt;
use std::collections::VecDeque;
use cursive::views::ViewRef;

use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum FieldType {
    Text,
    Number,
    Date,
    Time,
}

#[derive(Clone)]
struct FieldConfig {
    label: String,
    field_type: FieldType,
    max_length: usize,
}

impl FieldConfig {
    pub fn new() -> Self {
        FieldConfig {
            label: String::new(),
            field_type: FieldType::Text,
            max_length: 0,
        }
    }
}

#[derive(Clone)]
struct Fields{
    name: String,
    config: FieldConfig,
}

impl Fields {
    pub fn new() -> Self {
        Fields {
            name: String::new(),
            config: FieldConfig::new(),
        }
    }
}

pub struct ConfigurableForm {
    siv: Cursive,
    fields: VecDeque<Fields>,
    label_and_field: Fields,
}

impl ConfigurableForm {
    pub fn new() -> Self {
        ConfigurableForm {
            siv: Cursive::default(),
            fields: VecDeque::new(),
            label_and_field: Fields::new(),
        }
    }

    pub fn add_label(&mut self, label: &str) {
        self.label_and_field.config.label = label.to_string();
    }

    pub fn add_field(&mut self, id: String, field_type: FieldType, max_length: usize) {
        println!("Size: {}",max_length);        
        self.label_and_field.name = id.clone();
        self.label_and_field.config.field_type = field_type;
        self.label_and_field.config.max_length = max_length;
        self.fields.push_back(self.label_and_field.clone());
    }

    pub fn show(&mut self) {
        let mut layout = LinearLayout::vertical();
        let mut horizontal_layout;

        self.siv.load_toml(include_str!("../../assets/styles.toml")).unwrap();

        for field in &self.fields {
            let name_clone = field.name.clone();
            let name = field.name.to_string();
            let config = &field.config;

            horizontal_layout = LinearLayout::horizontal();
            horizontal_layout.add_child(TextView::new(&config.label).with_name(&config.label).fixed_width(config.label.chars().count()));
            println!("Label: '{}' - size: {}", config.label, config.label.len());

            match config.field_type {
                FieldType::Text => {
                    let edit_view = EditView::new();
                    horizontal_layout.add_child(edit_view.on_edit(move |siv, _content, cursor| on_edit_text(siv, &name_clone, cursor))
                        .with_name(name)
                        .fixed_width(config.max_length));
                }
                FieldType::Number => {
                    horizontal_layout.add_child(EditView::new()
                        .on_edit(move |siv, _content, cursor| on_edit_number(siv, &name_clone, cursor))
                        .with_name(name).fixed_width(config.max_length));
                }
                FieldType::Date => {
                    let view = EditDateView::new(&name, config.max_length);
                    horizontal_layout.add_child(view);
                }
                FieldType::Time => {
                    horizontal_layout.add_child(EditView::new()
                        .on_edit(move |siv, _content, cursor| on_edit_time(siv, &name_clone, cursor))
                        .with_name(name).fixed_width(config.max_length));
                }
            }

            layout.add_child(horizontal_layout
                    .fixed_width(config.label.len() + config.max_length));
        }

        let flds = self.fields.clone();
        self.siv.add_layer(Dialog::around(layout.with_name("layout")).title("Configurable Form")
            .button("Quit", Cursive::quit)
            .button("Submit", move |siv| on_submit_button(siv, &flds)));
        self.siv.run();
    }

}

fn on_submit_button(siv: &mut Cursive, fields: &VecDeque<Fields>){
    for fld in fields {
        let v: ViewRef<EditView> = siv.find_name(fld.name.as_str()).unwrap();
        println!("Fld: {} content:[{}]", fld.name, v.get_content());
    }
}

fn on_edit_text(siv: &mut Cursive, _content: &str, _cursor: usize) {
    let re = Regex::new(r"^[a-zA-Z0-9_\s]*$").unwrap();

    // Get handles for check input
    let text = siv.call_on_name(_content, |v: &mut EditView| v.get_content()).unwrap();

    // Checking
    if !re.is_match(&text) {
        siv.call_on_name(_content, |v: &mut EditView| v.set_content(text.chars().filter(|c| c.is_alphabetic()).collect::<String>()));
    }
}

fn on_edit_number(siv: &mut Cursive, _content: &str, _cursor: usize) {
    let re =  Regex::new(r"^[0-9]+$").unwrap();

    // Get handles for check input
    let text = siv.call_on_name(_content, |v: &mut EditView| v.get_content()).unwrap();

    // Checking
    if !re.is_match(&text) {
        siv.call_on_name(_content, |v: &mut EditView| {
            v.set_content(
                text.chars()
                    .filter(|c| c.is_digit(10)) // Allow only numeric characters
                    .collect::<String>(),
            )
        });
    }
}

//      ^                           # Start of string
//      (?:                         # Try to match...
//          (?:                     #   Try to match...
//              ([01]?\d|2[0-3]):   #       HH:
//          )?                      #   (optionally).
//          ([0-5]?\d):             #   MM: (required)
//      )?                          # (entire group optional, so either HH:MM:, MM: or nothing)
//      ([0-5]?\d)                  # SS (required)
//      $                           # End of string
fn on_edit_time(siv: &mut Cursive, _content: &str, _cursor: usize) {
    let re = Regex::new(r"^(?:(?:([01]?\d|2[0-3]):)?([0-5]?\d):)?([0-5]?\d)$").unwrap();

    // Get handles for check input
    let text = siv.call_on_name(_content, |v: &mut EditView| v.get_content()).unwrap();

    // Checking
    if !re.is_match(&text) {
        siv.call_on_name(_content, |v: &mut EditView| {
            v.set_content(
                text.chars()
                    .filter(|c| c.is_digit(10) || *c == ':') // Allow digits and colons
                    .collect::<String>(),
            )
        });
    }
}
