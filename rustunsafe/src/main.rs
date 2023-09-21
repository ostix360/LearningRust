use std::slice;

fn main() {
    let address = 0x012345usize;
    let r = address as *mut i32;

    let mut nb = 5;

    let r1 = &nb as *const i32;
    let r2 = &mut nb as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        let value = unsafe{ slice::from_raw_parts_mut(r,10000)};
    }
}

extern fn joe(){

}
