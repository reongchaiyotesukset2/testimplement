use std::cell::Cell;

struct structmain{
	 id : Cell<u32>,
	 name : Cell<String>,
}

impl structmain{

	  fn new()->Self{
   	  Self{
   	  	id : Cell::default(),
   	  	name  : Cell::default(),
   	  }
   }
   
   fn test1(&self){
   	   println!("{:#?}",self.name.clone());
   	  }
 }
fn main() {
    //how to Cell
    let ins = structmain::new();
    ins.test1();
}
