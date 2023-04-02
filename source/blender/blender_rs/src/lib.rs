#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "C" fn RUST_init(context: *mut bContext) {
    println!("RUST_init executed; quiting...");
    unsafe { WM_exit(context); }
}
