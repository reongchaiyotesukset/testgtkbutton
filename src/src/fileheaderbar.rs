use gtk::prelude::*;
use gdk::prelude::*;

use gtk::{HeaderBar};

pub struct makeheaderbar{

}
impl makeheaderbar{

  pub fn headerbar_config(button : &gtk::Button)-> HeaderBar{
         let headerbar = HeaderBar::builder()
            .title("Title ")
            .subtitle("sub test2")
            .show_close_button(true)
            .build();
           
     headerbar
   
}
}
