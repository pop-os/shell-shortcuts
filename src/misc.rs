use gtk::prelude::*;
use pango::Weight;

pub fn label_weight(label: &gtk::Label, weight: Weight) {
    let list = label.get_attributes().unwrap_or_else(pango::AttrList::new);
    list.insert(pango::Attribute::new_weight(weight).expect("new weight returned null"));
    label.set_attributes(Some(&list));
}
