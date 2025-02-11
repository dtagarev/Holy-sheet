use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Button, Label, Stack};
use super::AppPage;

#[derive(Clone)]
pub struct PageTwo {
    container: GtkBox,
    stack: Stack,
}

impl PageTwo {
    pub fn new(stack: Stack) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 8);

        let label = Label::new(Some("This is PAGE TWO"));
        container.append(&label);

        let back_btn = Button::with_label("Back to Main");
        {
            let stack_clone = stack.clone();
            back_btn.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("main");
            });
        }
        container.append(&back_btn);

        Self { container, stack }
    }
}

impl AppPage for PageTwo {
    fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}