use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Button, Label, Stack, TextView, ScrolledWindow, Entry};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use super::AppPage;

#[derive(Clone)]
pub struct EditPage {
    container: GtkBox,
    stack: Stack,
    text_view: TextView,
    entry: Entry,
}

impl EditPage {
    pub fn new(stack: Stack) -> Self {
        // Главен контейнер за вертикално подреждане
        let container = GtkBox::new(Orientation::Vertical, 8);

        // Заглавен текст
        let label = Label::new(Some("Edit your cheatsheet"));
        container.pack_start(&label, false, false, 0);

        // Поле за въвеждане на име на файла
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Enter file name"));
        container.pack_start(&entry, false, false, 0);

        // Създаваме TextView за редактиране на съдържанието
        let text_view = TextView::new();
        let scrolled = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrolled.add(&text_view);
        container.pack_start(&scrolled, true, true, 0);

        // Бутон за запазване
        let save_btn = Button::with_label("Save");
        {
            let entry_clone = entry.clone();
            let text_view_clone = text_view.clone();
            save_btn.connect_clicked(move |_| {
                let filename = entry_clone.text().to_string();
                let buffer = text_view_clone.buffer().unwrap();
                let content = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false).unwrap();
                if let Err(e) = save_file(&filename, &content) {
                    eprintln!("Failed to save file: {}", e);
                }
            });
        }
        container.pack_start(&save_btn, false, false, 0);

        Self { container, stack, text_view, entry }
    }
}

impl AppPage for EditPage {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}

/// Запазва съдържанието във файл в `~/.config/holy-sheet/cheatsheets/<filename>`.
fn save_file(filename: &str, content: &str) -> Result<(), String> {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir().join("cheatsheets");
    fs::create_dir_all(&config_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    let file_path = config_dir.join(format!("{}.md", filename));

    fs::write(&file_path, content).map_err(|e| format!("Failed to write to file: {}", e))?;
    Ok(())
}