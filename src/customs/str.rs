// For read: https://stackoverflow.com/questions/61378351/how-to-implement-a-trait

use regex::Regex;

pub trait CustomStr {
    fn clean_text(&self) -> String;
    fn is_match_rgx(&self, rx: &str) -> bool;
}

// Implementa el trait para &str
impl CustomStr for &str {
    fn clean_text(&self) -> String {
        let mut cleaned = String::new();
        for c in self.chars() {
            match c {
                'á' | 'é' | 'í' | 'ó' | 'ú' => cleaned.push(unaccent(c)),
                'Á' | 'É' | 'Í' | 'Ó' | 'Ú' => cleaned.push(unaccent(c.to_lowercase().next().unwrap())),
                'ñ' | 'Ñ' => cleaned.push('n'),
                '.' | ' ' | ':' => (), // Ignorar.
                _ => cleaned.push(c),
            }
        }
        cleaned.to_lowercase()
    }

    fn is_match_rgx(&self, rx: &str) -> bool {
        let re = Regex::new(rx).unwrap();
        re.is_match(self)
    }
}

fn unaccent(c: char) -> char {
    match c {
        'á' | 'Á' => 'a',
        'é' | 'É' => 'e',
        'í' | 'Í' => 'i',
        'ó' | 'Ó' => 'o',
        'ú' | 'Ú' => 'u',
        _ => c,
    }
}