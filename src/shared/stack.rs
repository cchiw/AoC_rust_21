pub struct Stack {
    pub stack : Vec<char>
}

impl Stack{
    pub fn new()->Self{
        Stack{stack: Vec::new()}
    }
    pub fn push(&mut self, item: char){
        self.stack.push(item)
    }
    pub fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }
    pub fn length(&self)-> usize{
        self.stack.len()
    }
    pub fn peek(&self) -> Option<char>{
    	match self.stack.last() {
    		None => None,
    		Some(c) => Some(*c),
    		}
    }
}
