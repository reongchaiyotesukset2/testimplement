
fn log_message(message: &str) {
     println!("[LOG] {}", message);
}


mod utils {

    pub fn calculate_sum(a: i32, b: i32) -> i32 {
        a + b
    }


pub mod calculations {
use log_message;   
use super::*;
 

use super::super::*;

        pub fn perform_calculation(x: i32, y: i32) {

            let result = calculate_sum(x, y);


         log_message(&format!("Calculation completed! Result: {}", result));
        }
    }
}

fn main() {
utils::calculations::perform_calculation(10, 5);
}
