use std::cell::Cell;


trait traitmain{
    fn getid();
    
}
struct structmain{
	 id : Cell<u32>,
	 name : Cell<String>,
}

impl structmain{
	
}

struct structparent{
	 address :String,
	 phonenumber : String,
}

impl structparent{
	
}
//split trait and structmain
impl traitmain for structmain{
 
	 fn getid(){
	 	
	 }
}
impl structmain{
	  fn new()->Self{
   	  Self{
   	  	id : Cell::default(),
   	  	name  : Cell::default(),
   	  }
   }
   
   fn test1(&self){
   	   println!("{:#?}",self.name);
   	  }
   }
fn main() {
    //how to Cell
    let ins = structmain::new();
    ins.test1();
}
