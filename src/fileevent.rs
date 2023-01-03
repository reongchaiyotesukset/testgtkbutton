use gtk::prelude::*;
use gtk::{MessageDialog, DialogFlags, MessageType,ButtonsType};
pub struct makeEvent{
	
}
impl makeEvent{	
	
pub fn event_click(button)
{
	button.connect_clicked(move |text| {
				       
let dialog = MessageDialog::new(
	None::<&gtk::Window>,
	DialogFlags::empty(),
	MessageType::Info,
	ButtonsType::Ok,
	"Hello World!",
);
	dialog.run();
	dialog.close();
});   
}	
}

