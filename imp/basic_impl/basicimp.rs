struct MainImp{
	 id : u32,
	 name : String,
}

impl MainImp{
	  fn new()->Self{
   	  Self{
   	  	id : 1,
   	  	name  : "jone".to_string(),
   	  }
   }
   
   fn test1(&self){
   	  	
    }
   }
fn main() {

    let ins = MainImp::new();
    ins.test1();
  
    
}
