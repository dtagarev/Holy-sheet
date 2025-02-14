use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Button, Label, Stack};
use super::AppPage;

#[derive(Clone)]
pub struct PageThree {
    container: GtkBox,
    stack: Stack,
}

impl PageThree {
    pub fn new(stack: Stack) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 8);

        let label = Label::new(Some("This is PAGE THREE"));
        container.pack_start(&label, false, false, 0);

        let back_btn = Button::with_label("Back to Main");
        {
            let stack_clone = stack.clone();
            back_btn.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("main");
            });
        }
        container.pack_start(&back_btn, false, false, 0);

        Self { container, stack }
    }
}

impl AppPage for PageThree {
    fn widget(&self) -> &gtk::Widget {
        self.container.upcast_ref()
    }
}