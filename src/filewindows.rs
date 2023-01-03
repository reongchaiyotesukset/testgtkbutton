extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application,Window, ApplicationWindow};

 use gtk::builders::ApplicationBuilder;

pub struct makeWindows{
	
}
impl makeWindows{	
	
	   pub fn window_config(app :ApplicationBuilder)-> ApplicationWindow{ 
		  
        
        let window = ApplicationWindow::builder()
             .application(app)
             .default_width(300)
             .default_height(200)
             .build();
             
		      window
		}
		
}


