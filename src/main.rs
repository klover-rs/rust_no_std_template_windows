#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;

mod print;

use print::print_impl;
use std_alloc::string::ToString;
use wee_alloc::WeeAlloc;

pub extern crate alloc as std_alloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT; // we need that as our allocation handler in no_std

use winapi::um::processthreadsapi::ExitProcess;

// we will create a macro for main because we fucking can 🔥 🔥 🔥
#[macro_export]
macro_rules! main {
    ($main_body:block) => {
        #[no_mangle]
        pub extern "C" fn main() -> ! {
            // Execute the main body code here, to capture its value in the variable
            let exit_code = {
                let exit_code = (|| {
                    $main_body
                })();

                exit_code
            };

            // will exit the process
            unsafe { ExitProcess(exit_code as u32) };

            loop {} // unreachable, but still needs to be there to give the compiler the expected return type !
        }
    };
}

main!({
    let a_vec = std_alloc::vec!["hello world!"]; // test the allocation by creating a fucking vec

    let string = a_vec[0].to_string(); // get the first index of the created vec and convert it to a string just for fun

    print!("{}", &string); 

    let mut counter = 0;
    loop {
        println!("counter: {}", counter);
        counter += 1;

        if counter >= 100000 {
            println!("finished printing!!");
            break;
        }
    }

    let formatted_msg = std_alloc::format!("abc: {}", 23);

    let large_number_vec: std_alloc::vec::Vec<i128> = std_alloc::vec![2342343243, 324234, 23842384723847234982348923, 23498072389472389472348972348923789423, 23484823984723723948723894772394872348];

    println!("{} uwu : {:?}", &formatted_msg, large_number_vec);

    return 0; // finished execution with code 0;
});
