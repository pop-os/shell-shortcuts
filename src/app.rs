use gtk::prelude::*;
use std::rc::Rc;

const LAPTOP_DARK: &[u8] = include_bytes!("../assets/laptop-dark.svg");
const DISPLAY_DARK: &[u8] = include_bytes!("../assets/display-dark.svg");

const CSS: &[u8] = br#"
.keyboard-key {
  color: #222;
  background: #ccc;
  padding: .25em .5em;
  border-radius: .3em;
  border-bottom: .2em inset #444;
}
"#;

const COLUMNS: &[&[Section]] = &[
    &[
        Section::new(
            "Move window across monitors",
            &[
                Shortcut::new(
                    "Move current window one monitor to the left",
                    Event::MoveMonitorLeft,
                    Schema::Hardcoded(&["Super", "Ctrl", "←"]),
                ),
                Shortcut::new(
                    "Move current window one monitor to the right",
                    Event::MoveMonitorRight,
                    Schema::Hardcoded(&["Super", "Ctrl", "→"]),
                ),
            ],
        ),
        Section::new(
            "Navigate applications, windows, and workspaces",
            &[
                Shortcut::new(
                    "Launch and switch applications",
                    Event::Search,
                    Schema::Hardcoded(&["Super", "/"]),
                ),
                Shortcut::new(
                    "Switch focus between windows",
                    Event::SwitchFocus,
                    Schema::Hardcoded(&["Super", "arrow key"]),
                ),
                Shortcut::new(
                    "Switch focus to monitor left",
                    Event::SwitchFocusMonitorLeft,
                    Schema::Hardcoded(&["Super", "Ctrl", "←"]),
                ),
                Shortcut::new(
                    "Switch focus to monitor right",
                    Event::SwitchFocusMonitorRight,
                    Schema::Hardcoded(&["Super", "Ctrl", "→"]),
                ),
                Shortcut::new(
                    "Switch focus to workspace above",
                    Event::SwitchFocusWorkspaceAbove,
                    Schema::Hardcoded(&["Super", "Ctrl", "↑"]),
                ),
                Shortcut::new(
                    "Switch focus to workspace below",
                    Event::SwitchFocusWorkspaceBelow,
                    Schema::Hardcoded(&["Super", "Ctrl", "↓"]),
                ),
            ],
        ),
    ],
    &[
        Section::new(
            "Move window across workspaces",
            &[
                Shortcut::new(
                    "Move current window one workspace up",
                    Event::MoveWorkspaceAbove,
                    Schema::Hardcoded(&["Super", "Shift", "↑"]),
                ),
                Shortcut::new(
                    "Move current window one workspace down",
                    Event::MoveWorkspaceBelow,
                    Schema::Hardcoded(&["Super", "Shift", "↓"]),
                ),
            ],
        ),
        Section::new(
            "Window adjustment",
            &[
                Shortcut::new(
                    "Enter adjustment mode",
                    Event::EnterAdjustment,
                    Schema::Hardcoded(&["Super", "Enter"]),
                ),
                Shortcut::new(
                    "Move window",
                    Event::MoveWindow,
                    Schema::Hardcoded(&["arrow key"]),
                ),
                Shortcut::new(
                    "Resize window",
                    Event::ResizeWindow,
                    Schema::Hardcoded(&["Shift", "arrow key"]),
                ),
                Shortcut::new(
                    "Snap windows",
                    Event::SnapWindows,
                    Schema::Hardcoded(&["Ctrl", "arrow key"]),
                ),
                Shortcut::new(
                    "Apply changes",
                    Event::ApplyChanges,
                    Schema::Hardcoded(&["Enter"]),
                ),
                Shortcut::new("Cancel", Event::Cancel, Schema::Hardcoded(&["Esc"])),
            ],
        ),
    ],
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    MoveMonitorLeft,
    MoveMonitorRight,
    Search,
    SwitchFocus,
    SwitchFocusMonitorLeft,
    SwitchFocusMonitorRight,
    SwitchFocusWorkspaceAbove,
    SwitchFocusWorkspaceBelow,
    MoveWorkspaceAbove,
    MoveWorkspaceBelow,
    EnterAdjustment,
    MoveWindow,
    ResizeWindow,
    SnapWindows,
    ApplyChanges,
    Cancel,
}

pub struct Section {
    pub header: &'static str,
    pub shortcuts: &'static [Shortcut],
}

impl Section {
    pub const fn new(header: &'static str, shortcuts: &'static [Shortcut]) -> Self {
        Self { header, shortcuts }
    }
}

pub struct Shortcut {
    pub description: &'static str,
    pub event: Event,
    pub schema: Schema,
}

impl Shortcut {
    pub const fn new(description: &'static str, event: Event, schema: Schema) -> Self {
        Self {
            description,
            event,
            schema,
        }
    }
}

pub enum Schema {
    // GSettings { key: &'static str, from: usize },
    Hardcoded(&'static [&'static str]),
}

pub fn main(app: &gtk::Application) {
    cascade! {
        provider: gtk::CssProvider::new();
        ..load_from_data(CSS).unwrap();
        | gtk::StyleContext::add_provider_for_screen(
            gdk::Screen::get_default().as_ref().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    }

    let laptop = &svg_draw_area(LAPTOP_DARK, 300, 230);
    let display = &svg_draw_area(DISPLAY_DARK, 300, 300);

    cascade! {
        gtk::ApplicationWindow::new(app);
        ..add(&cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 24);
            ..set_halign(gtk::Align::Center);
            ..set_border_width(8);
            ..add(&demo_section(&laptop, display));
            ..add(&shortcuts_section());
            ..add(&settings_reference());
        });
        ..show_all();
        ..connect_delete_event(move |window, _| {
            window.destroy();
            gtk::Inhibit(false)
        });
    }
}

fn svg_draw_area(svg: &[u8], width: i32, height: i32) -> gtk::DrawingArea {
    let drawing_area = gtk::DrawingArea::new();

    let opt = resvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(svg, &opt.usvg).unwrap();

    drawing_area.connect_draw(move |w, cr| {
        let screen = resvg::ScreenSize::new(
            w.get_allocated_width() as u32,
            w.get_allocated_height() as u32,
        )
        .unwrap();

        resvg::backend_cairo::render_to_canvas(&tree, &opt, screen, cr);

        gtk::Inhibit(false)
    });

    drawing_area.set_size_request(width, height);

    drawing_area
}

fn demo_section(laptop: &gtk::DrawingArea, display: &gtk::DrawingArea) -> gtk::Box {
    let container = cascade! {
        gtk::Box::new(gtk::Orientation::Horizontal, 32);
        ..set_halign(gtk::Align::Center);
        ..add(laptop);
        ..add(display);
    };

    laptop.set_valign(gtk::Align::End);

    container
}

fn settings_reference() -> gtk::Box {
    use std::process::Command;

    let hyperlink = cascade! {
        gtk::LinkButton::new("Keyboard Settings");
        ..connect_clicked(move |_| {
            let _ = Command::new("gnome-control-center")
                .arg("keyboard")
                .spawn();
        });
    };

    let container = cascade! {
        gtk::Box::new(gtk::Orientation::Horizontal, 0);
        ..set_halign(gtk::Align::Start);
        ..add(&cascade! {
            gtk::Label::new("These keyboard shortcuts can be changed in".into());
        });
        ..add(&hyperlink);
    };

    container
}

fn shortcuts_section() -> gtk::Box {
    let key_sg = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let event_handler: Rc<dyn Fn(&gtk::EventBox, Event)> = Rc::new(|widget, event| {
        println!("clicked {:?}", event);
    });

    for column in COLUMNS {
        let shortcuts = gtk::Box::new(gtk::Orientation::Vertical, 16);
        container.add(&shortcuts);

        for section in *column {
            let section = cascade! {
                crate::widgets::Section::new(&key_sg, section, &event_handler);
            };

            shortcuts.add(section.as_ref());
        }
    }

    container
}
