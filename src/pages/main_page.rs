use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Button, Label, Stack};
use super::AppPage;

#[derive(Clone)]
pub struct MainPage {
    container: GtkBox,
    stack: Stack,
}

impl MainPage {
    pub fn new(stack: Stack) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 8);

        let label = Label::new(Some("This is the MAIN page.\nChoose a sub-page:"));
        container.append(&label);

        // Бутон за PageOne
        let btn_one = Button::with_label("Go to Page ONE");
        {
            let stack_clone = stack.clone();
            btn_one.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("page_one");
            });
        }
        container.append(&btn_one);

        // Бутон за PageTwo
        let btn_two = Button::with_label("Go to Page TWO");
        {
            let stack_clone = stack.clone();
            btn_two.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("page_two");
            });
        }
        container.append(&btn_two);

        // Бутон за PageThree
        let btn_three = Button::with_label("Go to Page THREE");
        {
            let stack_clone = stack.clone();
            btn_three.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("page_three");
            });
        }
        container.append(&btn_three);

        Self { container, stack }
    }
}

impl AppPage for MainPage {
    fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}