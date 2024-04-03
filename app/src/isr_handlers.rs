
use core::panic::PanicInfo;

#[no_mangle]
#[panic_handler]
pub fn default_handler(_panic: &PanicInfo<'_>) -> ! {

    loop{}
}


#[no_mangle]
pub unsafe extern "C" fn reset_handler() -> ! {


    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;


