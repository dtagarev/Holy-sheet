use web_view::*;
use pulldown_cmark::{Parser, Options, html};
use directories::ProjectDirs;
use std::fs;
use std::env;

pub fn run_markdown_viewer(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    // Load the Markdown file and convert it to HTML
    let md_content = match load_markdown_file(file_name) {
        Ok(t) => t,
        Err(e) => format!("Error reading MD file:\n{}", e),
    };
    // let html_content = markdown_to_html(&md_content);

    let html_content = "<html><body><h1>Hello, World!</h1></body></html>";
    // Create the WebView
    // web_view::builder()
    //     .title("Holy Sheet - Markdown Viewer")
    //     .content(Content::Html(html_content))
    //     .size(800, 600)
    //     .resizable(true)
    //     .debug(true)
    //     .user_data(())
    //     .invoke_handler(|_webview, _arg| Ok(()))
    //     .run()?;
    web_view::builder()
    .title("My Project")
    .content(Content::Html(html_content))
    .size(320, 480)
    .resizable(false)
    .debug(true)
    .user_data(())
    .invoke_handler(|_webview, _arg| Ok(()))
    .run()
    .unwrap();

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