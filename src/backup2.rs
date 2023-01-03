extern crate gtk; 

use gtk::prelude::*; 

fn main() {
    // Initialize GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    } 

    // Create a new window
    let window = gtk::Window::new(gtk::WindowType::Toplevel); 

    // Set the window title
    window.set_title("GTK Entry and Button Example"); 

    // Set the window size
    window.set_default_size(300, 100); 

    // Create a new vertical box container
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5); 

    // Create a new GTK Entry
    let entry = gtk::Entry::new(); 

    // Add the entry to the vertical box container
    vbox.pack_start(&entry, true, true, 0); 

    // Create a new GTK Button
    let button = gtk::Button::new_with_label("Click me!"); 

    // Add the button to the vertical box container
    vbox.pack_start(&button, true, true, 0); 

    // Connect the button's "clicked" signal to a callback
    button.connect_clicked(move |_| {
        // Get the text from the entry
        let text = entry.get_text().expect("Failed to get text from GTK Entry."); 

        // Create a new GTK Dialog
        let dialog = gtk::MessageDialog::new(
            Some(&window),
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            &format!("You entered: {}", text),
        ); 

        // Show the dialog and wait for a response
        let response = dialog.run(); 

        // Destroy the dialog when it is closed
        dialog.destroy(); 

        // Handle the response
        match response {
            gtk::ResponseType::Ok => println!("Dialog closed with OK button."),
            _ => println!("Dialog closed with other button."),
        }
    }); 

    // Add the vertical box container to the window
    window.add(&vbox); 

    // Show the window and all of its child widgets
    window.show_all(); 

    // Run the GTK main loop
    gtk::main();
}
