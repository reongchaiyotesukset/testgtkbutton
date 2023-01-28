use gtk::prelude::*;
use gtk::{Layout};

pub struct makeLayout{
	
}
impl makeLayout{	
	
	   pub fn layout_config()-> Layout{ 
		  
        
        let layout = Layout::builder()
            .margin(0)
            .margin_bottom(0)
            .margin_top(0)
            .margin_start(0)
            .margin_end(0)
            .build();
        
		      layout
		    
		}
}

