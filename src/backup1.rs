				       
let dialog = MessageDialog::new(
	None::<&gtk::Window>,
	DialogFlags::empty(),
	MessageType::Info,
	ButtonsType::Ok,
	//"Hello World!",
	 &format!("You entered: {}", text),
);
	dialog.run();
	dialog.close();
