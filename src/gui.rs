use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};

pub struct GUI {
    window: Window,
    account_entry: Entry,
    unlock_button: Button,
    status_label: Label,
}

impl GUI {
    pub fn new() -> Self {
        gtk::init().expect("Failed to initialize GTK.");
        let window = Window::new(WindowType::Toplevel);
        let account_entry = Entry::new();
        let unlock_button = Button::with_label("Unlock Items");
        let status_label = Label::new(None);

        window.set_title("Hybrid - Unlock Fortnite Items");
        window.set_default_size(300, 200);
        window.set_border_width(10);
        window.add(&account_entry);
        window.add(&unlock_button);
        window.add(&status_label);

        window.show_all();
        Self { window, account_entry, unlock_button, status_label }
    }

    pub fn connect_signals<F>(&self, unlock_callback: F)
    where
        F: Fn(&str) + 'static,
    {
        let account_entry = self.account_entry.clone();
        self.unlock_button.connect_clicked(move |_| {
            let account_id = account_entry.get_text().unwrap_or_default();
            unlock_callback(&account_id);
        });
    }

    pub fn set_status(&self, message: &str) {
        self.status_label.set_text(message);
    }
}