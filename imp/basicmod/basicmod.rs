mod imp{


    use super::*;
    
    pub struct MainImp{
	 id : u32,
	 name : String,
    }

    impl MainImp{
        pub fn new()->Self{
            Self{
            id : 1,
            name  : "jone".to_string(),
            }
        }
   
   pub fn test1(&self){
      
   }
}
 
}   
fn main() {

        let ins = imp::MainImp::new();
        ins.test1();
    
    }
