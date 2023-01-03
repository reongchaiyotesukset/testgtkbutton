use gtk::prelude::*;
use gtk::{Entry};

pub struct makeEntry{
	
}
impl makeEntry{	
	
	   pub fn enrty_config()-> Entry{ 
		  
        
        let text = Entry::builder()
            .placeholder_text("input")
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
        
		     return text
		    
		}
}
