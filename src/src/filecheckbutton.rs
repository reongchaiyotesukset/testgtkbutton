use gtk::prelude::*;
use gtk::{ComboBox};

pub struct makeCheckButton{
	
}
impl makeCheckButton{	
	
	   pub fn Checkbutton_config1()-> gtk::CheckButton{ 
		  
         let  checkbox = gtk::CheckButton::builder()
                     .margin(140)
					 .margin_bottom(100)
					 .margin_top(140)
					 .margin_start(0)
					 .margin_end(0)
					 .label("TRUE")
					 .build();            
		      checkbox
		    
		}
		 pub fn Checkbutton_config2()-> gtk::CheckButton{ 
		  
         let  checkbox = gtk::CheckButton::builder()
                     .margin(116)
					 .margin_bottom(0)
					 .margin_top(140)
					 .margin_start(110)
					 .margin_end(100)
					 .label("FALSE")
					 .build();            
		      checkbox
		    
		}
}
