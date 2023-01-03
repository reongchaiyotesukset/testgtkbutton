// import gtk libs
extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application,Window, ApplicationWindow};
use gtk::{MessageDialog, DialogFlags, MessageType,ButtonsType,Entry};

mod filebutton;
mod filelayout;
//mod fileentry;

fn main() {
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();

app.connect_activate(|app| {

	let window = ApplicationWindow::builder()
	.application(app)
	.default_width(300)
	.default_height(200)
	.build();
	
	//let entry = Entry::new();
	//let entry = fileentry::makeEntry::enrty_config();
	  let text = Entry::builder()
            .placeholder_text("input")
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
	let button = filebutton::makeButton::button_config();
	let layout = filelayout::makeLayout::layout_config(); 
             layout.add(&text);
             layout.add(&button);
   
   //let text = entry.placeholder_text().expect("REASON").to_string(); 
	button.connect_clicked(move |_ิtext| {

               println!("{}",text.text().as_str());

     }); 
           

             window.add(&layout);
             window.show_all();
    });
   // Run..............................................................................................
    app.run();
   
}
