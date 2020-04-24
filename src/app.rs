use gtk::prelude::*;
use std::rc::Rc;

const LAPTOP_DARK: &[u8] = include_bytes!("../assets/laptop-dark.svg");
const DISPLAY_DARK: &[u8] = include_bytes!("../assets/display-dark.svg");

const CSS: &[u8] = br#"
"#;

const COLUMNS: &[&[Section]] = &[
    &[
        Section::new(
            "Direction keys",
            &[
                Shortcut::new(
                    "Arrow keys",
                    Event::ArrowKeys,
                    Schema::Hardcoded(&["←", "↓", "↑", "→"]),
                ),
                Shortcut::new(
                    "Vim shortcuts",
                    Event::ArrowKeysAlt,
                    Schema::Hardcoded(&["H", "J", "K", "L"]),
                ),
            ],
        ),
        Section::new(
            "Navigate applications and windows",
            &[
                Shortcut::new(
                    "Launch and switch applications",
                    Event::Search,
                    Schema::Hardcoded(&["Super", "/"]),
                ),
                Shortcut::new(
                    "Switch focus between windows",
                    Event::SwitchFocus,
                    Schema::Hardcoded(&["Super", "Direction keys"]),
                ),
            ],
        ),
        Section::new(
            "Move, resize and swap windows",
            &[
                Shortcut::new(
                    "Enter adjustment mode",
                    Event::EnterAdjustment,
                    Schema::Hardcoded(&["Super", "Enter"]),
                ),
                Shortcut::new(
                    "Move window",
                    Event::MoveWindow,
                    Schema::Hardcoded(&["Direction keys"]),
                ),
                Shortcut::new(
                    "Resize window",
                    Event::ResizeWindow,
                    Schema::Hardcoded(&["Shift", "Direction keys"]),
                ),
                Shortcut::new(
                    "Swap windows",
                    Event::SwapWindows,
                    Schema::Hardcoded(&["Ctrl", "Direction keys"]),
                ),
                Shortcut::new(
                    "Apply changes",
                    Event::ApplyChanges,
                    Schema::Hardcoded(&["Enter"]),
                ),
                Shortcut::new("Cancel",
                Event::Cancel,
                Schema::Hardcoded(&["Esc"]),
                ),
            ],
        ),
        Section::new(
            "Workspaces",
            &[
                Shortcut::new(
                    "Move current window up one workspace",
                    Event::MoveWorkspaceAbove,
                    Schema::Hardcoded(&["Super", "Shift", "↑"]),
                ),
                Shortcut::new(
                    "Move current window down one workspace",
                    Event::MoveWorkspaceBelow,
                    Schema::Hardcoded(&["Super", "Shift", "↓"]),
                ),
                Shortcut::new(
                    "Switch focus to the worksapce above",
                    Event::MoveWorkspaceAbove,
                    Schema::Hardcoded(&["Super", "Ctrl", "↑"]),
                ),
                Shortcut::new(
                    "Switch focus to the workspace below",
                    Event::MoveWorkspaceBelow,
                    Schema::Hardcoded(&["Super", "Ctrl", "↓"]),
                ),
            ],
        ),
        Section::new(
            "Window Shortcuts",
            &[
                Shortcut::new(
                    "Change window orientation",
                    Event::OrientationToggle,
                    Schema::Hardcoded(&["Super", "O"]),
                ),
                Shortcut::new(
                    "Toggle floating mode",
                    Event::FloatingToggle,
                    Schema::Hardcoded(&["Super", "G"]),
                ),
                Shortcut::new(
                    "Close window",
                    Event::CloseWindow,
                    Schema::Hardcoded(&["Super", "Q"]),
                ),
                Shortcut::new(
                    "Toggle maximize",
                    Event::MaximizeToggle,
                    Schema::Hardcoded(&["Super", "M"]),
                ),
            ],
        ),
        Section::new(
            "Launcher Shortcuts",
            &[
                Shortcut::new(
                    "Activate Launcher",
                    Event::Search,
                    Schema::Hardcoded(&["Super", "/"]),
                ),
                Shortcut::new(
                    "Execute a command in a terminal",
                    Event::ExecuteCommandTerminal,
                    Schema::Hardcoded(&["t:"]),
                ),
                Shortcut::new(
                    "Execute a command in sh",
                    Event::ExecuteCommandSh,
                    Schema::Hardcoded(&[":"]),
                ),
                Shortcut::new(
                    "Calculate an equation",
                    Event::Calculate,
                    Schema::Hardcoded(&["="]),
                ),
            ],
        ),
    ],
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    ArrowKeys,
    ArrowKeysAlt,
    MoveMonitorLeft,
    MoveMonitorRight,
    Search,
    ExecuteCommandTerminal,
    ExecuteCommandSh,
    Calculate,
    SwitchFocus,
    SwitchFocusMonitorLeft,
    SwitchFocusMonitorRight,
    SwitchFocusWorkspaceAbove,
    SwitchFocusWorkspaceBelow,
    MoveWorkspaceAbove,
    MoveWorkspaceBelow,
    EnterAdjustment,
    MoveWindow,
    ResizeWindowIncrease,
    ResizeWindowDecrease,
    SwapWindows,
    ApplyChanges,
    Cancel,
    FloatingToggle,
    OrientationToggle,
    MaximizeToggle,
    CloseWindow,
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

    let shortcuts = cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 24);
        ..set_border_width(8);
        //..add(&legend());
        ..add(&shortcuts_section());
        //..add(&settings_reference());
    };

    let scroller = cascade! {
        gtk::ScrolledWindowBuilder::new()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        ..set_vexpand(true);
        ..add(&shortcuts);
    };

    let content = cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 24);
        ..set_border_width(8);
        //..add(&demo_section(&laptop, display));
        ..add(&scroller);
    };

    let window = cascade! {
        gtk::ApplicationWindow::new(app);
        ..set_size_request(600, 500);
        ..set_icon_name("input-keyboard".into());
        ..add(&content);
        ..show_all();
        ..connect_delete_event(move |window, _| {
            window.destroy();
            gtk::Inhibit(false)
        });
    };

    window.connect_size_allocate(move |_, allocation| {
        let width = (allocation.width - allocation.width.min(1000)) / 2;

        content.set_margin_start(width);
        content.set_margin_end(width);
        shortcuts.set_halign(if width == 0 {
            gtk::Align::Center
        } else {
            gtk::Align::Fill
        });
    });
}

fn legend() -> gtk::Box {
    let arrow_keys = cascade! {
        gtk::Box::new(gtk::Orientation::Horizontal, 24);
        ..add(&cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 4);
            ..add(&crate::misc::keycap("←"));
            ..add(&crate::misc::keycap("↓"));
            ..add(&crate::misc::keycap("↑"));
            ..add(&crate::misc::keycap("→"));
        });
        ..add(&gtk::Label::new("- arrow keys".into()));
    };

    let alt_arrow_keys = cascade! {
        gtk::Box::new(gtk::Orientation::Horizontal, 24);
        ..add(&cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 4);
            ..add(&crate::misc::keycap("H"));
            ..add(&crate::misc::keycap("J"));
            ..add(&crate::misc::keycap("K"));
            ..add(&crate::misc::keycap("L"));
        });
        ..add(&gtk::Label::new("- use in place of arrow keys".into()));
    };

    let container = cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 8);
        ..add(&arrow_keys);
        ..add(&alt_arrow_keys);
    };

    container
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

fn shortcuts_section() -> gtk::FlowBox {
    let key_sg = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    let container = cascade! {
        gtk::FlowBox::new();
        ..set_selection_mode(gtk::SelectionMode::None);
        ..set_max_children_per_line(2);
        ..set_row_spacing(12);
        ..set_column_spacing(12);
    };

    let event_handler: Rc<dyn Fn(&gtk::EventBox, Event)> = Rc::new(|widget, event| {
        println!("clicked {:?}", event);
    });

    let iter = COLUMNS.iter().flat_map(|column| {
        column.iter().map(|section| {
            let section = cascade! {
                crate::widgets::Section::new(&key_sg, section, &event_handler);
            };

            section
        })
    });

    for widget in iter {
        container.add(widget.as_ref());
    }

    container
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
