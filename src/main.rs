extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;

use gtk::{Application,Window, ApplicationWindow};
use gtk::{MessageDialog, DialogFlags, MessageType,ButtonsType,Entry};

mod filewindows;
mod filebutton;
mod filelayout;
mod fileentry;
mod filecombobox;

fn main() {
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
app.connect_activate(|app| {
        let window = filewindows::Makewindow::window_config(app);
        let text = fileentry::makeEntry::enrty_config();
        let button = filebutton::makeButton::button_config();
        let combo = filecombobox::makeComboBox::combo_config();
        let layout = filelayout::makeLayout::layout_config();
        layout.add(&text);
        layout.add(&button);
        layout.add(&combo);
        button.connect_clicked(move |_text| {
			
              let dialog = MessageDialog::new(
                                        None::<&gtk::Window>,
                                        DialogFlags::empty(),
                                        MessageType::Info,
                                        ButtonsType::Ok,
                                        &format!("You entered: {}", text.text().as_str()),
                          );
                                 dialog.run();
                                 dialog.close();
     });
              combo.connect_changed(|combo| filecombobox::makeComboBox::combo_changed(combo));
             window.add(&layout);
             window.show_all();
    });
    app.run();

}
