use winapi::um::fileapi::WriteFile;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::wincon::SetConsoleOutputCP;
use winapi::{shared::ntdef::HANDLE, um::winbase::STD_OUTPUT_HANDLE};
use winapi::um::processenv::GetStdHandle;
use winapi::um::winnls::CP_UTF8;

use core::sync::atomic::{AtomicBool, Ordering};

static INIT: AtomicBool = AtomicBool::new(false);


fn set_console_output_cp_to_utf8() -> bool {
    unsafe {
        let handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE {
            return false;
        }

        let result = SetConsoleOutputCP(CP_UTF8);
        result != 0
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            let formatted_msg = $crate::std_alloc::format!($($arg)*);
            $crate::print_impl(&formatted_msg);
        }
    };
}


#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            let mut formatted_msg = $crate::std_alloc::format!($($arg)*);
            formatted_msg.push('\n');
            $crate::print_impl(&formatted_msg);
        }
    };
}

pub fn print_impl(message: &str) {
    use core::ptr;

    if !INIT.load(Ordering::SeqCst) {
        if INIT.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            let _ = set_console_output_cp_to_utf8();
        }
    } // sets the console to utf8, executes only one time!

    unsafe {
        let handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE {
            return;
        }

        let mut written = 0;
        let _ = WriteFile(
            handle,
            message.as_ptr() as *const _,
            message.len() as u32,
            &mut written,
            ptr::null_mut(),
        );
        
    }
}
