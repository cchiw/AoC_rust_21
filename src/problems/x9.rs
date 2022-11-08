use crate::shared::readings::read_grid;
use std::collections::HashSet;
use std::collections::HashMap;

const INPUT: &str = "src/data/input_x9.txt";
type Ty = usize;
type Tyv = i32;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: Ty,
    pub y: Ty,
}
impl Point {
    pub fn new(x: Ty, y: Ty) -> Self {
        Point { x: x, y: y }
    }
}
pub fn is_low(
    x_coord: Ty,
    y_coord: Ty,
    x_length: Ty,
    y_length: Ty,
    grid: &mut Vec<Vec<Tyv>>,
) -> bool {
    let current = grid[x_coord][y_coord];
    if x_coord >= 1 && grid[x_coord - 1][y_coord] <= current {
        return false;
    }
    if y_coord >= 1 && grid[x_coord][y_coord - 1] <= current {
        return false;
    }
    if x_coord + 1 < x_length && grid[x_coord + 1][y_coord] <= current {
        return false;
    }
    if y_coord + 1 < y_length && grid[x_coord][y_coord + 1] <= current {
        return false;
    }
    //print!("\n Low point {:?} by {:?} is {:?}", x_coord, y_coord, current);
    return true;
}

pub fn get_low_points(grid: &mut Vec<Vec<Tyv>>, low_points: &mut HashSet<Point>) {
    read_grid(INPUT, grid);
    let a_length = grid.len();
    let b_length = grid[0].len();
    let mut xi = 0;
    while xi < a_length {
        let mut yi = 0;
        while yi < b_length {
            if is_low(xi, yi, a_length, b_length, grid) {
                let p = Point::new(xi.try_into().unwrap(), yi.try_into().unwrap());
                low_points.insert(p);

            }
            yi += 1;
        }
        xi += 1;
    }
}

pub fn p1() {
    let mut grid = Vec::new();
    let mut low_points = HashSet::new();
    get_low_points(&mut grid, &mut low_points);
    let mut total = 0; //491
    for p in low_points{
        total += grid[p.x][p.y] + 1;
    }
    print!("total {:?}", total);
}
//6 9 8
//3 V1 2


pub fn grab_adj(ai:Ty, bi:Ty,     x_length: Ty,
    y_length: Ty, basin:&mut Vec<Vec<Option<Ty>>>)-> Option<Ty>{
	 let l = if ai>0 {basin[ai-1][bi]}else{None};
	 match l {
	 	None => (),
	 	Some(0) => (),
	  Some(lp) => return Some(lp),
	 }
	 let r = if ai+1<x_length {basin[ai+1][bi]}else{None};
	 match r {
	 	None => (),
	 	Some(0) => (),
	  Some(lp) => return Some(lp),
	 }
	 let t = if bi>0 {basin[ai][bi-1]}else{None};
	 match t {
	 	None => (),
	 	Some(0) => (),
	  Some(lp) => return Some(lp),
	 }
	 let b = if bi+1<y_length {basin[ai][bi+1]}else{None};
	 match b {
	 	None => return None,
	 	Some(0) => return None,
	  Some(lp) => return Some(lp),
	 }	 
}

pub fn basin_print(basin:&mut Vec<Vec<Option<Ty>>>){
 let mut ai = 0;
 while ai<basin.len(){
      let mut bi = 0;
      println!("");
      while bi<basin[0].len(){
      	print!("\t{:?}", basin[ai][bi]);
      	bi+=1;
      }
			ai+=1;
	}
}
pub fn p2() {
    let mut grid = Vec::new();
    let mut low_points = HashSet::new();
    let mut basin = Vec::new();
    get_low_points(&mut grid, &mut low_points);   
    let a_length = grid.len();
    let b_length = grid[0].len();
    
    let mut sizes = HashMap::new();
    
    let mut ai = 0;
    let mut count = 0;
    while ai<a_length{
      let mut bvec = Vec::new();
      let mut bi = 0;
 			while bi<b_length{
 			  let p = Point::new(ai, bi);
 				let val = 
 					if low_points.contains(&p){count+=1;sizes.insert(count,1);Some(count)} 
 					else if grid[ai][bi]==9{Some(0)}
 					else{None};
 				bvec.push(val);
 				bi+=1;
 			}	
 			basin.push(bvec);
 			ai+=1;
    }
 
 	  let mut changed = true;
    	while changed {
    		changed=false;
    		let mut ai = 0;
    		while ai<a_length{
      		let mut bi = 0;
 					while bi<b_length{ 
 					  //print!("\n\nSizes {:?}", sizes);
 			  		let val = match basin[ai][bi]{
 			  			Some(k) => Some(k),
 			  			None => {
 			  				let k=grab_adj(ai, bi,a_length, b_length, &mut basin);
 			  			  match k {
 			  			  	Some(i)=> {
 			  			  	    //print!("\n\ti {:?}", i);
 			  			  			let cnt = sizes.get(&i).unwrap();
 			  			  			sizes.insert(i, cnt+1);
 			  			  			changed=true; 
 			  			  		} ,
 			  			  	 _ => (),
 			  			 		}
 			  			  k
 			  				},
 			  			};
 			  		basin[ai][bi]= val;
    				bi+=1;
    			}
      	ai+=1;
    		}
    		//print!("\n\n---------------------\n");
    		//basin_print(&mut basin);
    }
    print!("\n Sizes{:?}", sizes);
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for (_, v) in sizes{
 			if v>max1{
 				max3 = max2;
 				max2 = max1;
 				max1 =v;
 			}
 			else if v>max2{
 				max3 = max2;
 				max2 = v;
 			} 
 			else if v>max3{
 				max3=v;
 			}  
    }
    print!("maximals {:?} {:?} {:?}", max1, max2, max3);
     print!("maximal sum {:?}", max1*max2*max3);
}
