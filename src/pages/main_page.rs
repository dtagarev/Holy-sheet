use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Button, Label, Stack, Image};
use super::AppPage;

#[derive(Clone)]
pub struct MainPage {
    container: GtkBox,
    stack: Stack,
}

impl MainPage {
    pub fn new(stack: Stack) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 8);

        // Add the logo
        let logo = Image::from_file("Logo/Logo.svg");
        container.pack_start(&logo, false, false, 0);

        let label = Label::new(Some("Holy sheet"));
        let label2 = Label::new(Some("The ultimate cheatsheet manager"));
        label.style_context().add_class("title"); 
        label2.style_context().add_class("subtitle"); 
        container.pack_start(&label, false, false, 0);
        container.pack_start(&label2, false, false, 0);

        // ConfigurationsPage button
        let btn_two = Button::with_label("Go to CheatSheets");
        btn_two.style_context().add_class("button");
        {
            let stack_clone = stack.clone();
            btn_two.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("configurations_page");
            });
        }
        container.pack_start(&btn_two, false, false, 0);

        // EditPage button
        let btn_edit = Button::with_label("Create/Edit Cheatsheet");
        btn_edit.style_context().add_class("button");
        {
            let stack_clone = stack.clone();
            btn_edit.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("edit_page");
            });
        }
        container.pack_start(&btn_edit, false, false, 0);

        // Add a spacer element at the bottom
        let spacer = GtkBox::new(Orientation::Vertical, 0);
        spacer.set_size_request(-1, 40); // Set the height of the spacer
        container.pack_start(&spacer, false, false, 0);

        Self { container, stack }
    }
}

impl AppPage for MainPage {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}