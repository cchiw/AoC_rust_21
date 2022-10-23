use std::str::Chars;

pub struct BST {
    pub occurances: u32,
    pub zeros: Option<Box<BST>>,
    pub ones: Option<Box<BST>>,
}
impl BST {
    pub fn new() -> Self {
        BST {
            occurances: 0,
            zeros: None,
            ones: None,
        }
    }
    pub fn insert(&mut self, mut elements: Chars) {
        self.occurances += 1;
        if let Some(element) = elements.next() {
            match element {
                '0' => match &mut self.zeros {
                    None => {
                        let mut b = Box::new(BST::new());
                        b.insert(elements);
                        self.zeros = Some(b);
                    }
                    Some(b) => {
                        b.insert(elements);
                    }
                }, //end of zeros match
                '1' => match &mut self.ones {
                    None => {
                        let mut b = Box::new(BST::new());
                        b.insert(elements);
                        self.ones = Some(b);
                    }
                    Some(b) => {
                        b.insert(elements);
                    }
                }, //end of ones match
                _ => panic!("unexpected character"),
            } //end of element match
        } //end of processing next element
    } //end insert
    pub fn get_zeros_value(self) -> u32 {
        match self.zeros {
            None => 0,
            Some(b) => b.occurances,
        }
    }
    pub fn get_ones_value(self) -> u32 {
        match self.ones {
            None => 0,
            Some(b) => b.occurances,
        }
    }
    pub fn walk_max(&self, bits: &mut Vec<u32>) {
        match (&self.zeros, &self.ones) {
            (None, None) => {}
            (Some(z), None) => {
                println!("\n --Only zero");
                bits.push(0);
                z.walk_max(bits);
            }
            (None, Some(o)) => {
                println!("\n -- Only One");
                bits.push(1);
                o.walk_max(bits);
            }
            (Some(z), Some(o)) => {
                println!(
                    "\n -- comparing 0 bits ({:?}) 1 bits ({:?})",
                    &z.occurances, &o.occurances
                );
                if z.occurances > o.occurances {
                    bits.push(0);
                    print!("\t Decided on zero");
                    z.walk_max(bits);
                } else if z.occurances < o.occurances {
                    bits.push(1);
                    print!("\t Decided on one");
                    o.walk_max(bits);
                } else {
                    bits.push(1);
                    print!("\t It's a tie Decided on one");
                    o.walk_max(bits);
                } // end else
            } //end pattern
        } //end match
    } // end walk Max
    pub fn walk_min(&self, bits: &mut Vec<u32>) {
        match (&self.zeros, &self.ones) {
            (None, None) => {}
            (Some(z), None) => {
                bits.push(0);
                z.walk_min(bits);
            }
            (None, Some(o)) => {
                bits.push(1);
                o.walk_min(bits);
            }
            (Some(z), Some(o)) => {
                if z.occurances <= o.occurances {
                    bits.push(0);
                    z.walk_min(bits);
                } else {
                    bits.push(1);
                    o.walk_min(bits);
                } // end else
            } //end pattern
        } //end match
    } // end walk Max
    pub fn get_most_bin(&self) -> u32 {
        let mut t0: Vec<u32> = Vec::new();
        self.walk_max(&mut t0);
        println!("\n\n Believes Max vec is {:?}", t0);
        let t1 = t0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("");
        u32::from_str_radix(&t1, 2).unwrap()
    }
    pub fn get_least_bin(&self) -> u32 {
        let mut t0: Vec<u32> = Vec::new();
        self.walk_min(&mut t0);
        println!("Believes Min vec is {:?}", t0);
        let t1 = t0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("");
        u32::from_str_radix(&t1, 2).unwrap()
    }
} //end impl
