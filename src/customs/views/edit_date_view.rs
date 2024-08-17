use cursive::views::{EditView, OnEventView, NamedView, ResizedView};
use cursive::event::{Event, Key};
use cursive::view::{ViewWrapper, Nameable, Resizable};
use cursive::{wrap_impl};
use crate::customs::views::date::{on_edit_date, callback_del};

pub struct EditDateView {
    name_view: String,
    inner: OnEventView<ResizedView<NamedView<EditView>>>,
}

impl EditDateView {
    pub fn new(name: &str, len: usize) -> Self {
        let nc = name.to_string().clone();
        let mut view = OnEventView::new(EditView::new()
            .on_edit(move |siv, _content, cursor| on_edit_date(siv, &nc, cursor))
                    .with_name(name)
                    .fixed_width(len));
        
        view.set_on_pre_event(Event::Key(Key::Del), callback_del);
        
        EditDateView {
            name_view: name.to_string(),
            inner: view,
        }
    }
}

impl ViewWrapper for EditDateView  {
    wrap_impl!(self.inner: OnEventView<ResizedView<NamedView<EditView>>>);
}
