use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Button, Label, Stack, TextView, ScrolledWindow, Entry};
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
        container.append(&label);

        // Поле за въвеждане на име на файла
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Enter file name"));
        container.append(&entry);

        // Създаваме TextView за редактиране на съдържанието
        let text_view = TextView::new();
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scrolled.set_child(Some(&text_view));
        container.append(&scrolled);

        // Бутон за запазване
        let save_btn = Button::with_label("Save");
        {
            let stack_clone = stack.clone();
            let text_view_clone = text_view.clone();
            let entry_clone = entry.clone();
            save_btn.connect_clicked(move |_| {
                let file_name = entry_clone.text().to_string();
                let content = text_view_clone.buffer().text(&text_view_clone.buffer().start_iter(), &text_view_clone.buffer().end_iter(), false).to_string();
                if let Err(e) = save_file(&file_name, &content) {
                    eprintln!("Error saving file: {}", e);
                } else {
                    stack_clone.set_visible_child_name("main");
                }
            });
        }
        container.append(&save_btn);

        // Бутон "Назад"
        let back_btn = Button::with_label("Back to Main");
        {
            let stack_clone = stack.clone();
            back_btn.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("main");
            });
        }
        container.append(&back_btn);

        Self { container, stack, text_view, entry }
    }
}

impl AppPage for EditPage {
    fn widget(&self) -> &gtk4::Widget {
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