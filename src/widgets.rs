use crate::{
    app::{Schema, Section as SectionInfo},
    misc::keycap,
};

use gio::prelude::*;
use gtk::prelude::*;
use std::collections::HashMap;

use crate::app::State;

#[derive(AsRef, Deref)]
#[as_ref]
#[deref]
pub struct Section(gtk::Box);

impl Section {
    pub fn new(
        key_sg: &gtk::SizeGroup,
        section: &SectionInfo,
        state: &mut State,
        settings_map: &mut HashMap<&'static str, gio::Settings>,
    ) -> Self {
        let label = gtk::LabelBuilder::new()
            .label(section.header)
            .xalign(0.0)
            .build();

        crate::misc::label_weight(&label, pango::Weight::Bold);

        let bindings = gtk::Box::new(gtk::Orientation::Vertical, 8);

        for shortcut in section.shortcuts {
            let orientation = match shortcut.schema {
                Schema::GSettings { .. } => gtk::Orientation::Vertical,
                Schema::Hardcoded(_) => gtk::Orientation::Horizontal,
            };
            let keys = gtk::Box::new(orientation, 4);

            key_sg.add_widget(&keys);

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

            event_box.connect_button_press_event(move |_, _| {
                // TODO: Someday handle click event
                gtk::Inhibit(true)
            });

            match shortcut.schema {
                Schema::GSettings { schema, key } => {
                    let settings = settings_map.entry(schema)
                        .or_insert_with(|| crate::app::open_schema(schema));

                    for i in settings.get_strv(key) {
                        keys.add(&gtk::ShortcutLabel::new(&i));
                    }

                    state.register(schema.into(), settings.clone(), key.into(), keys);
                }
                Schema::Hardcoded(binding) => {
                    binding.iter().for_each(|binding| {
                        keys.add(&keycap(binding));
                    });
                }
            }            

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
