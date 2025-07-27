#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness:u8
}

impl Light {
    pub fn new(alias: &str) -> Self{
      Self{
          
      alias: alias.to_string(),
      brightness:0
      }
    }
}

pub fn change_brightness(slc: &mut [Light], alias: &str, val: u8){
     slc
        .iter_mut().for_each(|l| 
            {
                if l.alias == alias{
                
                l.brightness=val 
            } 
            } );
        }
        
        
        
        // println!("{:#?} ******* {} +++++++ {}", l, alias, value));