use crate::{
    app::{Event, Schema, Section as SectionInfo},
    misc::keycap,
};

use gtk::prelude::*;
use std::rc::Rc;

#[derive(AsRef, Deref)]
#[as_ref]
#[deref]
pub struct Section(gtk::Box);

impl Section {
    pub fn new(
        key_sg: &gtk::SizeGroup,
        section: &SectionInfo,
        func: &Rc<dyn Fn(&gtk::EventBox, Event)>,
    ) -> Self {
        let label = gtk::LabelBuilder::new()
            .label(section.header)
            .xalign(0.0)
            .build();

        crate::misc::label_weight(&label, pango::Weight::Bold);

        let bindings = gtk::Box::new(gtk::Orientation::Vertical, 8);

        for shortcut in section.shortcuts {
            let keys = gtk::Box::new(gtk::Orientation::Horizontal, 4);

            key_sg.add_widget(&keys);

            match shortcut.schema {
                Schema::Hardcoded(binding) => {
                    binding.iter().for_each(|binding| {
                        keys.add(&keycap(binding));
                    });
                }
            }

            let event_box = gtk::EventBoxBuilder::new()
                .can_focus(false)
                .hexpand(true)
                .events(gdk::EventMask::BUTTON_PRESS_MASK)
                .build();

            event_box.add(&cascade! {
                gtk::Box::new(gtk::Orientation::Horizontal, 12);
                ..add(&keys);
                ..add(&gtk::Label::new(shortcut.description.into()));
            });

            event_box.connect_button_press_event(enclose!((func) move |event_box, _| {
                func(event_box, shortcut.event);
                gtk::Inhibit(true)
            }));

            bindings.add(&event_box);
        }

        Section(cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 12);
            ..set_size_request(400, -1);
            ..add(&label);
            ..add(&bindings);
        })
    }
}
