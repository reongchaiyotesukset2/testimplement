trait Traitmain{
    fn getid();
}
struct Structmain{
	 id : u32,
	 name : String,
}
impl Structmain{}
impl Traitmain for structmain{
	  fn getid(){
	 	  println!("getid");
	 }
}
impl Structmain{
	  fn new()->Self{
   	  Self{
   	  	id :  1,
   	  	name : "name".to_string(),
   	  }
   }
   fn test1(&self){
   	println!("test1");
   	  self.getid();
   }
 }
   
fn main() {
  let ins = Structmain::new();
  ins.test1();
}
