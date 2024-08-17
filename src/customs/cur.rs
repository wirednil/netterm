
struct CustomCursive {
    list_name: LinkedList<&'static str>,
}

pub trait CustomCursive {
    fn new() -> Self;
    fn add_name(nm: &String);
    fn get_list_fld_name() -> LinkedList<String>;

}

impl CustomCursive for Cursive {
    fn new() -> CustomCursive|
}
