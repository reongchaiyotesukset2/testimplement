use std::cell::RefCell;

#[derive(Debug)] // เพิ่ม derive(Debug) เพื่อให้สามารถพิมพ์ค่า Provider ได้ หากต้องการ
struct Provider {
    id: u32,
}
impl Provider {

    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

struct Aa {
    pub selected_provider: RefCell<Option<Provider>>,
}

impl Aa {
    pub fn new() -> Self {
        Self {
            // RefCell::default() สำหรับ Option<T> จะคืนค่า RefCell(None)
            selected_provider: RefCell::default(),
        }
    }
    
    // เมธอดสำหรับกำหนดค่า Provider
    pub fn set_provider(&self, provider: Provider) {
        // ใช้ replace() เพื่อนำค่าใหม่เข้าไปแทนที่ค่าเก่า (ซึ่งตอนนี้คือ None)
        // หรือใช้ *self.selected_provider.borrow_mut() = Some(provider);
        self.selected_provider.replace(Some(provider)); 
    }
    
    pub fn test(&self) {
        // ในบล็อกนี้เราแค่ 'ยืม' ค่าเพื่ออ่านเท่านั้น
        if let Some(provider) = self.selected_provider.borrow().as_ref() {
            println!("ok - Provider ID: {}", provider.id);
        } else {
            println!("else - No provider selected");
        }
    }
}

fn main() {
    println!("Hello, world!");
    let ins = Aa::new();
    
    // 1. ทดสอบก่อนกำหนดค่า (จะได้ผลลัพธ์: else)
    println!("--- Test 1 (Before setting) ---");
    ins.test(); 

    // 2. กำหนดค่า Provider เข้าไปใน RefCell
    let new_provider = Provider::new(42);
    ins.set_provider(new_provider);
    
    // 3. ทดสอบหลังกำหนดค่า (จะได้ผลลัพธ์: ok)
    println!("--- Test 2 (After setting) ---");
    ins.test();
}
