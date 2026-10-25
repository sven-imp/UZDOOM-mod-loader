use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, FileChooserButton, FileChooserAction, Label, Orientation, Entry};
use std::process::Command;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

const APP_ID: &str = "com.example.UZDoomLoader";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

/// Ensure the file has executable permission; if not, try to add it
fn ensure_executable(path: &str) -> Result<(), String> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    let metadata = std::fs::metadata(p).map_err(|e| e.to_string())?;
    let mut perms = metadata.permissions();
    // Check if any execute bit is set (user/group/other)
    if perms.mode() & 0o111 == 0 {
        // Try to add execute permission for the owner
        perms.set_mode(perms.mode() | 0o100);
        std::fs::set_permissions(p, perms)
            .map_err(|e| format!("Failed to set executable permission: {}", e))?;
    }
    Ok(())
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

    let bin_label = Label::new(Some("UZDoom Executable (binary or AppImage):"));
    bin_label.set_halign(gtk::Align::Start);
    let bin_chooser = FileChooserButton::new("Select UZDoom Binary", FileChooserAction::Open);
    vbox.pack_start(&bin_label, false, false, 0);
    vbox.pack_start(&bin_chooser, false, false, 0);

    let iwad_label = Label::new(Some("IWAD (e.g., doom2.wad):"));
    iwad_label.set_halign(gtk::Align::Start);
    let iwad_chooser = FileChooserButton::new("Select IWAD", FileChooserAction::Open);
    vbox.pack_start(&iwad_label, false, false, 0);
    vbox.pack_start(&iwad_chooser, false, false, 0);

    let mods_label = Label::new(Some("Mod Files (separate multiple paths with ';'):"));
    mods_label.set_halign(gtk::Align::Start);
    let mods_entry = Entry::new();
    mods_entry.set_placeholder_text(Some("/path/to/mod1.pk3;/path/to/mod2.wad"));
    vbox.pack_start(&mods_label, false, false, 0);
    vbox.pack_start(&mods_entry, false, false, 0);

    let launch_btn = Button::with_label("Launch UZDoom");
    vbox.pack_start(&launch_btn, false, false, 10);

    let status_label = Label::new(None);
    vbox.pack_start(&status_label, false, false, 0);

    let bin_chooser_clone = bin_chooser.clone();
    let iwad_chooser_clone = iwad_chooser.clone();
    let mods_entry_clone = mods_entry.clone();
    let status_label_clone = status_label.clone();

    launch_btn.connect_clicked(move |_| {
        let bin_path = bin_chooser_clone.filename();
        let iwad_path = iwad_chooser_clone.filename();

        let bin = match bin_path {
            Some(p) => p.to_string_lossy().to_string(),
            None => {
                status_label_clone.set_text("Error: Please select the UZDoom binary.");
                return;
            }
        };

        let iwad = match iwad_path {
            Some(p) => p.to_string_lossy().to_string(),
            None => {
                status_label_clone.set_text("Error: Please select an IWAD file.");
                return;
            }
        };

        // Key fix: ensure the UZDoom binary has executable permission (AppImage usually needs chmod +x)
        if let Err(e) = ensure_executable(&bin) {
            status_label_clone.set_text(&format!("Permission error: {}", e));
            return;
        }

        let mods_raw = mods_entry_clone.text().to_string();
        let mods: Vec<&str> = mods_raw
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut cmd = Command::new(&bin);
        cmd.arg("-iwad").arg(&iwad);

        if !mods.is_empty() {
            cmd.arg("-file");
            for m in &mods {
                cmd.arg(m);
            }
        }

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
