#![no_std]
#![no_main]
 
psp::module!("Tutorial", 1, 0);
 
pub fn psp_main() {
    psp::enable_home_button();
 
    psp::dprintln!("Hello World!");
}