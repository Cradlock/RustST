

pub trait Runnable {
    fn initialize(&self) -> Result<String, String>;
    fn run(&self);
}


