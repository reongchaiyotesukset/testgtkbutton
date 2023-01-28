use gtk::prelude::*;
use gtk::{Label};

pub struct makeLable{
	
}
impl makeLable{	
	
  
	   pub fn Lable_config()-> Label{ 
        let label = Label::builder()
            .margin(0)
            .margin_bottom(0)
            .margin_top(33)
            .margin_start(15)
            .margin_end(0)
            .label("name")
            .build();
		    label
		    
		}
}
