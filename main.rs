use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, FileChooserButton, FileChooserAction, Label, Orientation, Entry};
use std::process::Command;

const APP_ID: &str = "com.example.UZDoomLoader";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("UZDoom Mod Loader")
        .default_width(600)
        .default_height(400)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);

    // UZDoom Binary Selector
    let bin_label = Label::new(Some("UZDoom Executable:"));
    bin_label.set_halign(gtk::Align::Start);
    let bin_chooser = FileChooserButton::new("Select UZDoom Binary", FileChooserAction::Open);
    vbox.pack_start(&bin_label, false, false, 0);
    vbox.pack_start(&bin_chooser, false, false, 0);

    // IWAD Selector
    let iwad_label = Label::new(Some("IWAD (e.g., doom2.wad):"));
    iwad_label.set_halign(gtk::Align::Start);
    let iwad_chooser = FileChooserButton::new("Select IWAD", FileChooserAction::Open);
    vbox.pack_start(&iwad_label, false, false, 0);
    vbox.pack_start(&iwad_chooser, false, false, 0);

    // Mods Entry (comma or space separated paths)
    let mods_label = Label::new(Some("Mod Files (semicolon ';' separated):"));
    mods_label.set_halign(gtk::Align::Start);
    let mods_entry = Entry::new();
    mods_entry.set_placeholder_text(Some("/path/to/mod1.pk3;/path/to/mod2.wad"));
    vbox.pack_start(&mods_label, false, false, 0);
    vbox.pack_start(&mods_entry, false, false, 0);

    // Launch Button
    let launch_btn = Button::with_label("Launch UZDoom");
    vbox.pack_start(&launch_btn, false, false, 10);

    // Status Label
    let status_label = Label::new(None);
    vbox.pack_start(&status_label, false, false, 0);

    // Connect the button click
    let bin_chooser_clone = bin_chooser.clone();
    let iwad_chooser_clone = iwad_chooser.clone();
    let mods_entry_clone = mods_entry.clone();
    let status_label_clone = status_label.clone();

    launch_btn.connect_clicked(move |_| {
        let bin_path = bin_chooser_clone.filename();
        let iwad_path = iwad_chooser_clone.filename();

        let bin = match bin_path {
            Some(p) => p,
            None => {
                status_label_clone.set_text("Error: Please select the UZDoom binary.");
                return;
            }
        };

        let iwad = match iwad_path {
            Some(p) => p,
            None => {
                status_label_clone.set_text("Error: Please select an IWAD file.");
                return;
            }
        };

        // Parse mods: split by semicolon and trim whitespace
        let mods_raw = mods_entry_clone.text().to_string();
        let mods: Vec<&str> = mods_raw
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        // Build the command
        let mut cmd = Command::new(&bin);
        cmd.arg("-iwad").arg(&iwad);

        if !mods.is_empty() {
            cmd.arg("-file");
            for m in &mods {
                cmd.arg(m);
            }
        }

        // Spawn the process
        match cmd.spawn() {
            Ok(_) => {
                status_label_clone.set_text("UZDoom launched successfully.");
            }
            Err(e) => {
                status_label_clone.set_text(&format!("Failed to launch: {}", e));
            }
        }
    });

    window.set_child(Some(&vbox));
    window.show_all();
}
