use crate::{
    app::{Event, Schema, Section as SectionInfo},
    misc::keycap,
};

use gio::prelude::*;
use glib::clone;
use glib::translate::{FromGlibPtrContainer, ToGlibPtr};
use gtk::prelude::*;
use std::collections::HashMap;
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

            match shortcut.schema {
                Schema::GSettings { schema, key } => {
                    let settings =
                        settings_map.entry(schema).or_insert_with(|| {
                            if schema == "org.gnome.shell.extensions.pop-shell" {
                                let settings_schema = gio::SettingsSchemaSource::from_directory(
                                "/usr/share/gnome-shell/extensions/pop-shell@system76.com/schemas",
                                None,
                                false
                            ).unwrap().lookup(schema, false).unwrap();
                                gio::Settings::new_full::<gio::SettingsBackend>(
                                    &settings_schema,
                                    None,
                                    None,
                                )
                            } else {
                                gio::Settings::new(schema)
                            }
                        });

                    for i in settings.get_strv(key) {
                        keys.add(&gtk::ShortcutLabel::new(&i));
                        //let (keysym, mask) = gtk::accelerator_parse(&i);
                        //keys.add(&keycap(&gtk::accelerator_get_label(keysym, mask).unwrap()));
                    }
                    let action = settings.create_action(key).unwrap();
                    action.connect_property_state_notify(clone!(@weak keys => @default-panic, move |action| {
                        let state = action.get_state().unwrap();
                        let strv: Vec<glib::GString> = unsafe { FromGlibPtrContainer::from_glib_container(glib_sys::g_variant_get_strv(state.to_glib_none().0, std::ptr::null_mut())) };
                        keys.foreach(|w| keys.remove(w));
                        for i in strv {
                            keys.add(&gtk::ShortcutLabel::new(&i));
                            //let (keysym, mask) = gtk::accelerator_parse(&i);
                            //keys.add(&keycap(&gtk::accelerator_get_label(keysym, mask).unwrap()));
                        }
                        keys.show_all();

                    }));

                    // Trick so that `action` is freed when the `keys` GObject is freed
                    unsafe { keys.set_data("action", action) };
                }
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
