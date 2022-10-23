type Ty = i32;

pub struct Stack {
    stack : Vec<Ty>;
}

impl Stack{
    fn new()->Self{
        Stack{stack: Vec::new()}
    }

    fn push(&mut self, item:Ty) -> Self{
        self.stack.push(item)
    }
    fn pop(&mut self) -> Ty {
        self.stack.pop()
    }
    fn length(&self)-> usize{
        self.stack.len()
    }
}
