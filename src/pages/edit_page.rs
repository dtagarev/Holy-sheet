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
    app: gtk::Application,
}

impl EditPage {
    pub fn new(stack: Stack, app: gtk::Application) -> Self {
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

        // Бутон за връщане към главната страница
        let back_btn = Button::with_label("Back to Main");
        {
            let stack_clone = stack.clone();
            back_btn.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("main");
            });
        }
        container.pack_start(&back_btn, false, false, 0);

        // Бутон за преглед
        let preview_btn = Button::with_label("Preview");
        {
            let app_clone = app.clone();
            let text_view_clone = text_view.clone();
            preview_btn.connect_clicked(move |_| {
                let buffer = text_view_clone.buffer().unwrap();
                let content = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false).unwrap();
                if let Err(e) = crate::pages::markdown_viewer::preview_markdown_content(&app_clone, &content) {
                    eprintln!("Failed to show markdown viewer: {}", e);
                }
            });
        }
        container.pack_start(&preview_btn, false, false, 0);
        Self { container, stack, text_view, entry, app }
    }

    pub fn load_configuration(&self, filename: &str) {
        let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
            .expect("Could not locate config directory");
        let config_dir = proj_dirs.config_dir().join("cheatsheets");
        let file_path = config_dir.join(format!("{}.md", filename));

        if let Ok(content) = fs::read_to_string(&file_path) {
            if let Some(buffer) = self.text_view.buffer() {
                buffer.set_text(&content);
            }
            self.entry.set_text(filename);
        } else {
            eprintln!("Failed to load configuration: {}", filename);
        }
    }
}

impl AppPage for EditPage {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}

/// Save teh content to a file in `~/.config/holy-sheet/cheatsheets/<filename>`.
fn save_file(filename: &str, content: &str) -> Result<(), String> {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir().join("cheatsheets");
    fs::create_dir_all(&config_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    let file_path = config_dir.join(format!("{}.md", filename));

    fs::write(&file_path, content).map_err(|e| format!("Failed to write to file: {}", e))?;
    Ok(())
}