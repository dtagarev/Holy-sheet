use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Button, Label, Stack, ListBox, ListBoxRow};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use super::{AppPage, EditPage};

#[derive(Clone)]
pub struct ConfigurationsPage {
    container: GtkBox,
    stack: Stack,
    edit_page: EditPage,
}

impl ConfigurationsPage {
    pub fn new(stack: Stack, edit_page: EditPage) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 8);

        let label = Label::new(Some("Your Configurations"));
        container.pack_start(&label, false, false, 0);

        let list_box = ListBox::new();
        if let Ok(configs) = get_configurations() {
            for config in configs {
                let row = ListBoxRow::new();
                let button = Button::with_label(&config);
                {
                    let stack_clone = stack.clone();
                    let config_clone = config.clone();
                    let edit_page_clone = edit_page.clone();
                    button.connect_clicked(move |_| {
                        stack_clone.set_visible_child_name("edit_page");
                        edit_page_clone.load_configuration(&config_clone);
                    });
                }
                row.add(&button);
                list_box.add(&row);
            }
        }
        container.pack_start(&list_box, true, true, 0);

        let back_btn = Button::with_label("Back to Main");
        {
            let stack_clone = stack.clone();
            back_btn.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("main");
            });
        }
        container.pack_start(&back_btn, false, false, 0);

        Self { container, stack, edit_page }
    }
}

impl AppPage for ConfigurationsPage {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}

fn get_configurations() -> Result<Vec<String>, String> {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir().join("cheatsheets");

    let mut configs = Vec::new();
    for entry in fs::read_dir(config_dir).map_err(|e| format!("Failed to read config directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        if let Some(file_name) = entry.path().file_stem() {
            if let Some(file_name) = file_name.to_str() {
                configs.push(file_name.to_string());
            }
        }
    }
    Ok(configs)
}