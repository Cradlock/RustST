use crate::common;


pub struct Collector {
    commands: Vec<Box<dyn common::Runnable>>,    
}

impl Collector {
        
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }


    pub fn select(&self, arg: &String) -> Result<&dyn common::Runnable, String> {

    }

        
}


