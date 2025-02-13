use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, ScrolledWindow};
use webkit2gtk::{WebView, WebViewExt};
use directories::ProjectDirs;
use pulldown_cmark::{Parser, Options, html};
use std::fs;

pub fn run_webview_mode(file_name: &str) -> Result<()> {
    let app = Application::new(
        Some("com.example.holy-sheet.webview"),
        Default::default(),
    );

    // Клонираме file_name, ако ще го ползваме в closure
    let file_name = file_name.to_string();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Holy Sheet - WebView Only"));
        window.set_default_size(800, 600);

        // 1) Зареждаме Markdown файл, подаден през file_name
        let md_content = match load_markdown_file(&file_name) {
            Ok(t) => t,
            Err(e) => format!("Error reading MD file:\n{}", e),
        };

        // 2) Преобразуваме в HTML
        let html_content = markdown_to_html(&md_content);

        // 3) WebView widget
        let web_view = WebView::new();
        web_view.load_html(&html_content, None);

        // 4) Слагаме WebView в ScrolledWindow
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        // scrolled.set_child(Some(&web_view));

        // 5) Слагаме scrolled в прозореца
        window.set_child(Some(&scrolled));
        window.show();
    });

    app.run();
    Ok(())
}

fn load_markdown_file(filename: &str) -> Result<String, String> {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir();
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