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

        let label = Label::new(Some("Holy sheet\nThe ultimate cheatsheet manager"));
        container.pack_start(&label, false, false, 0);

        // Бутон за PageOne
        let btn_one = Button::with_label("Go to Page ONE");
        btn_one.style_context().add_class("button");
        {
            let stack_clone = stack.clone();
            btn_one.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("page_one");
            });
        }
        container.pack_start(&btn_one, false, false, 0);

        // Бутон за ConfigurationsPage
        let btn_two = Button::with_label("Go to CheatSheets");
        btn_two.style_context().add_class("button");
        {
            let stack_clone = stack.clone();
            btn_two.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("configurations_page");
            });
        }
        container.pack_start(&btn_two, false, false, 0);

        // Бутон за PageThree
        let btn_three = Button::with_label("Go to Page THREE");
        btn_three.style_context().add_class("button");
        {
            let stack_clone = stack.clone();
            btn_three.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("page_three");
            });
        }
        container.pack_start(&btn_three, false, false, 0);

        // Бутон за EditPage
        let btn_edit = Button::with_label("Create/Edit Cheatsheet");
        btn_edit.style_context().add_class("button");
        {
            let stack_clone = stack.clone();
            btn_edit.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("edit_page");
            });
        }
        container.pack_start(&btn_edit, false, false, 0);

        Self { container, stack }
    }
}

impl AppPage for MainPage {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}