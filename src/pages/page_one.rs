use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Button, Label, Stack, TextView, ScrolledWindow};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs;

use super::AppPage;

#[derive(Clone)]
pub struct PageOne {
    container: GtkBox,
    stack: Stack,
}

impl PageOne {
    pub fn new(stack: Stack) -> Self {
        // Главен контейнер за вертикално подреждане
        let container = GtkBox::new(Orientation::Vertical, 8);
        // Заглавен текст
        let label = Label::new(Some("This is PAGE ONE (Cheatsheet)"));
        container.append(&label);

        // Зареждаме съдържанието на markdown файла
        // (примерно "page_one.md")
        let md_content = match load_markdown_file("test.md") {
            Ok(text) => text,
            Err(e) => format!("Error loading markdown file: {}", e),
        };

        // Създаваме TextView и слагаме текста
        let text_view = TextView::new();
        if let buffer = text_view.buffer() {
            buffer.set_text(&md_content);
        }
        // Правим го read-only
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
         // Слагаме TextView в ScrolledWindow, за да може да се скролва
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scrolled.set_child(Some(&text_view));

        container.append(&scrolled);
        // Бутон "Назад"
        let back_btn = Button::with_label("Back to Main");
        {
            let stack_clone = stack.clone();
            back_btn.connect_clicked(move |_| {
                // Връщаме се към главната страница
                stack_clone.set_visible_child_name("main");
            });
        }
        container.append(&back_btn);
        Self { container, stack }
    }
}

impl AppPage for PageOne {
    fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

/// Зарежда файла `~/.config/holy-sheet/<filename>` и връща като String.
fn load_markdown_file(filename: &str) -> Result<String, String> {
    // Ползваме directories::ProjectDirs, за да намерим ~/.config/holy-sheet
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet/cheatsheets")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir();
    let file_path = config_dir.join(filename);

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read {:?}: {}", file_path, e))?;
    Ok(content)
}
