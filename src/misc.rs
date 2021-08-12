use gtk::prelude::*;
use pango::Weight;

pub fn keycap(label: &str) -> gtk::Label {
    let label = cascade! {
        gtk::LabelBuilder::new().label(label).build();
        ..style_context().add_class("keycap");
    };

    label
}

pub fn label_weight(label: &gtk::Label, weight: Weight) {
    let list = label.attributes().unwrap_or_else(pango::AttrList::new);
    list.insert(pango::Attribute::new_weight(weight));
    label.set_attributes(Some(&list));
}
