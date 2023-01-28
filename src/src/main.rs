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
mod filecheckbutton;
mod filelabel;
mod fileheaderbar;

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
        let checkbox1 = filecheckbutton::makeCheckButton::Checkbutton_config1();
        let checkbox2 = filecheckbutton::makeCheckButton::Checkbutton_config2();
        let lable1 = filelabel::makeLable::Lable_config();
        let headerbar1 = fileheaderbar::makeheaderbar::headerbar_config(&button);
        window.set_titlebar(Some(&headerbar1));
        layout.add(&lable1);
        layout.add(&text);
        layout.add(&button);
        layout.add(&combo);
        layout.add(&checkbox1);
        layout.add(&checkbox2);
        
 //modify
 
         checkbox1.connect_toggled(move |checkbox1| {
        if checkbox1.is_active() {
            println!("CheckButton 1 is selected.");
        } else {
            println!("CheckButton 1 is deselected.");
        }
    });
    
		  checkbox2.connect_toggled(move |checkbox2| {
        if checkbox2.is_active() {
            println!("CheckButton 2 is selected.");
        } else {
            println!("CheckButton 2 is deselected.");
        }
    });
 //end
		
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
