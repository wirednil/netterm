use cursive::event::Event;
use cursive::event::Key;
use regex::Regex;
use cursive::Cursive;
use cursive::views::EditView;
use cursive::theme::Color;
use cursive::theme::ColorStyle;
use cursive::theme::BaseColor;

pub fn on_edit_date(siv: &mut Cursive, _content: &str, _cursor: usize) {
    // https://stackoverflow.com/questions/15491894/regex-to-validate-date-formats-dd-mm-yyyy-dd-mm-yyyy-dd-mm-yyyy-dd-mmm-yyyy
    let re = Regex::new(r"^((((0[1-9]|1[0-9]|2[0-8])[\/](0[1-9]|1[012]))|((29|30|31)[\/](0[13578]|1[02]))|((29|30)[\/](0[4,6,9]|11)))[\/]?(19|[2-9][0-9])?\d{0,2}$)|(^29[\/]02[\/]?(19|[2-9][0-9])?(00|04|08|12|16|20|24|28|32|36|40|44|48|52|56|60|64|68|72|76|80|84|88|92|96)?$)").unwrap();
    
    println!("A ver:");
    siv.set_global_callback(Event::Key(Key::Del), callback_del);
    println!("A ver: cont: {}", _content);

    // Get handles for check input
    let text = siv.call_on_name(_content, |v: &mut EditView| v.get_content()).unwrap();
    
    println!("A ver:");
    // Checking
    if !re.is_match(&text) {
        println!("ERROR: Input: {} size {}", text, text.len());
        siv.call_on_name(_content, |v: &mut EditView| {
            v.set_style(ColorStyle::new(
                Color::Light(BaseColor::Red),
                Color::TerminalDefault,
            ));
            let mut nuevo_cursor = _cursor;
            v.set_content(mask_date_time(&text, &mut nuevo_cursor));
            v.set_cursor(nuevo_cursor);
        });
    } else {
        println!("OK: Input: {} size {}", text, text.len());
        siv.call_on_name(_content, |v: &mut EditView| {
            v.set_style(ColorStyle::new(
                Color::Light(BaseColor::Blue),
                Color::TerminalDefault,
            ));
            let mut nuevo_cursor = _cursor.clone();
            v.set_content(mask_date_time(&text, &mut nuevo_cursor));
            v.set_cursor(nuevo_cursor);
        });
    }
}

// DD/MM/YYYY
fn mask_date_time(input: &str, _cursor: &mut usize) -> String {
    println!("Ini \\-----------------");
    println!("Input len: {} - Current cursor: {}", input.len(), _cursor);
    let mut result = String::new();
    let mut change = true;
    if input.is_empty() {
        result.push_str("");
        return result;
    }

    let mut day: i32 = -1;
    let mut month = -1;
    let chars: Vec<char> = input.chars().filter(|c| c.is_digit(10)).collect();

    for i in 0..(chars.len()-0) {
        println!("///FOR::{}", i);
        let c = chars[i];

        match i {
            0 => {
                println!("\t\t{}) char: {} - input: {}",i, c, input);
                if c > '3' {
                    println!("\t\tRetornar false: chars[{}]", c);
                    *_cursor -= 1;
                    change = false;
                } else {
                    result.push(c);
                }
                continue;
            },
		    1 => {
                println!("\t\t{}) char: {} - input: {}",i, c, input);
                day = convert_to_day(chars[i-1], c); 
                if day < 1 || day > 31 {
                    println!("\t\tRetornar false: day={}", day);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
                // break;
            },
		    2 => {
                println!("\t\t{}) char: {} - input: {}",i, c, input);
                if c != '1' && c != '0' {
                    println!("\t\tRetornar false: chars[{}]", c);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
			    continue;
            },
		    3 => {
                println!("\t\t{}) char: {} - input: {}",i, c, input);
                month = convert_to_month(chars[i-1],c);
                if month < 1 || month > 12 {
                    println!("\t\tRetornar false: month={}", month);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
                // break;
			},
            4 => {
                if c < '1' || c > '9' {
                    println!("\t\tRetornar false: year={}", c);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
            },
            5 => {
                let yy = convert_to_day(chars[i-1], c); 
                if yy < 19 || yy > 99 {
                    println!("\t\tRetornar false: month={}", yy);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
            },
            6 => {
                if c < '0' || c > '9' {
                    println!("\t\tRetornar false: year={}", c);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
            },
            7 => {
                let yy = convert_to_day(chars[i-1], c); 
                if yy < 0 || yy > 99 {
                    println!("\t\tRetornar false: month={}", yy);
                    *_cursor -= if *_cursor == 0 {0} else {1} ;
                    change = false;
                } else {
                    result.push(c);
                }
            },
            _ => (),
        }
    }

    println!("Pre-format: {}", result);

    // Insertar separador de fechas y mover el cursor hacia adelante.
    if result.len() > 2  {
        result.insert(2, '/');
        if input.chars().nth(2).unwrap() != '/' {
            *_cursor += 1;
        } 
        if result.len() > 5 {
            result.insert(5, '/');
            if input.chars().nth(5).unwrap() != '/' {
                *_cursor += 1;
            } 
        }
    }

    // Manejo de cursores, cuando se hace backspace.
    if (input.len() - 1) == result.len() && change {
        *_cursor -= if *_cursor == 0 {0} else {1} ;
    }

    println!("Fin-ret: {}.len({}) - current cursor: {}", result,result.len(), _cursor);
    result
}


fn convert_to_day(t: char, u: char) -> i32 {
    
    let tens  = t.to_string().parse::<i32>().unwrap();
    let units = u.to_string().parse::<i32>().unwrap();

    // Combinar los dígitos para formar el número
    let day = (tens * 10 + units) as u8;
    println!("({} * 10 = {}) + {} = {}", tens, tens*10, units, day);
    i32::from(day)
}

fn convert_to_month(t: char, u: char) -> i32 {
    let tens  = t.to_string().parse::<i32>().unwrap();
    let units = u.to_string().parse::<i32>().unwrap();

    // Combinar los dígitos para formar el número
    let month = (tens * 10 + units) as u8;
    println!("({} * 10 = {}) + {} = {}", tens, tens*10, units, month);

    i32::from(month)
}

pub fn callback_del(s: &mut Cursive){
    println!("Ignorando del")
}

pub fn callback_bs(s: &mut Cursive){
    println!("Ignorando backspace");
}