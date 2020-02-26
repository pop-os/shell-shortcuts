#[macro_use]
extern crate cascade;
#[macro_use]
extern crate enclose;
#[macro_use]
extern crate derive_more;

mod app;
mod misc;
mod widgets;

use gio::prelude::*;
use gtk::prelude::*;

const APP_ID: &str = "com.system76.ShellShortcuts";

fn main() {
    glib::set_program_name("Keyboard Shortcuts".into());
    glib::set_application_name("Keyboard Shortcuts");

    let app_flags = gio::ApplicationFlags::empty();

    cascade! {
        gtk::Application::new(APP_ID.into(), app_flags).expect("GTK initialization failed");
        ..connect_startup(move |app| app::main(&app));
        ..connect_activate(|app| {
            let windows = app.get_windows();
            if !windows.is_empty() {
                windows[0].present();
            }
        });
        ..run(&[]);
    }
}
