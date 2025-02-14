use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation, Button, Label};
use webkit2gtk::{WebView, WebViewExt};
use directories::ProjectDirs;
use pulldown_cmark::{Parser, Options, html};
use std::fs;

pub fn run_markdown_viewer(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::new(
        Some("com.example.holy-sheet.markdown_viewer"),
        Default::default(),
    );

    let file_name = file_name.to_string();
    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Holy Sheet - Markdown Viewer");
        window.set_default_size(800, 600);

        // Главен контейнер за вертикално подреждане
        let container = GtkBox::new(Orientation::Vertical, 8);

        // Заглавен текст
        let label = Label::new(Some("Markdown Viewer"));
        container.pack_start(&label, false, false, 0);

        // Зареждаме съдържанието на markdown файла
        let md_content = match load_markdown_file(&file_name) {
            Ok(text) => text,
            Err(e) => format!("Error loading markdown file: {}", e),
        };

        // Създаваме WebView и зареждаме HTML съдържанието
        let web_view = WebView::new();
        let html_content = markdown_to_html(&md_content);
        web_view.load_html(&html_content, None);

        // Слагаме WebView директно в контейнера
        container.pack_start(&web_view, true, true, 0);

        // Бутон "Назад"
        let back_btn = Button::with_label("Back to Main");
        let app_clone = app.clone();
        back_btn.connect_clicked(move |_| {
            app_clone.quit();
        });
        container.pack_start(&back_btn, false, false, 0);

        window.add(&container);
        window.show_all();
    });

    app.run();
    Ok(())
}

fn load_markdown_file(filename: &str) -> Result<String, String> {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir().join("cheatsheets");
    let file_path = config_dir.join(filename);

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read {:?}: {}", file_path, e))?;
    Ok(content)
}

fn markdown_to_html(md_text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(md_text, options);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    let css = r#"
    <style>
        body {
            font-family: sans-serif;
            margin: 1rem;
        }
        h1, h2, h3 {
            color: #2d76d9;
        }
        code {
            background: #f4f4f4;
            padding: 2px 4px;
            border-radius: 4px;
        }
    </style>
    "#;

    format!(
        r#"<!DOCTYPE html>
<html>
<head>{css}</head>
<body>
{content}
</body>
</html>
"#,
        css = css,
        content = html_buf
    )
}