use directories::ProjectDirs;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::fs;
use webkit2gtk::WebView;
use webkit2gtk::WebViewExt;

pub fn setup_markdown_viewer(app: &Application, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Set the environment variable to disable DMA-BUF renderer
    env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let window = ApplicationWindow::new(app);
    window.set_title("Holy Sheet - Markdown Viewer");
    window.set_default_size(800, 600);


    let html_content = match load_markdown_file(file_name) {
        Ok(content) => markdown_to_html(&content),
        Err(_) => "<p>File does not exist</p>".to_string(),
    };

    let webview = WebView::new();
    webview.load_html(&html_content, None);

    window.add(&webview);
    window.show_all();

    Ok(())
}

pub fn run_markdown_viewer(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    gtk::init().expect("Failed to initialize GTK");

    let application = Application::new(
        Some("com.markdown.holy-sheet"),
        Default::default(),
    );

    let file_name = file_name.to_string();
    application.connect_activate(move |app| {

        if let Err(e) = setup_markdown_viewer(app, &file_name) {
            eprintln!("Error setting up markdown viewer: {:?}", e);
        }
    });

    application.run();

    Ok(())
}

pub fn preview_markdown_content(app: &Application, md_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variable to disable DMA-BUF renderer
    env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let window = ApplicationWindow::new(app);
    window.set_title("Holy Sheet - Markdown Viewer");
    window.set_default_size(800, 600);

    let html_content = markdown_to_html(md_content);

    let webview = WebView::new();
    webview.load_html(&html_content, None);

    window.add(&webview);
    window.show_all();

    Ok(())
}

fn load_markdown_file(filename: &str) -> Result<String, String> {
    let proj_dirs = ProjectDirs::from("com", "markdown", "holy-sheet")
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

    let css = fs::read_to_string("src/pages/style/dark_theme.css")
        .expect("Failed to read CSS file");

    format!(
        r#"<!DOCTYPE html>
        <html>
        <head><style>{css}</style></head>
        <body>
        {content}
        </body>
        </html>
        "#,
        css = css,
        content = html_buf
    )
}