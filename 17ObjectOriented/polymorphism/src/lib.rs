pub trait Draw {
    fn draw(&self);
}

pub struct Screen{
    pub component:Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.component.iter(){
            component.draw();
        }
    }
}