use std::{cell::RefCell, vec};

thread_local! {
    #[doc(hidden)]
    pub static INPUT: RefCell<Vec<u8>> = Default::default();
    #[doc(hidden)]
    pub static OUTPUT: RefCell<vec::IntoIter<u8>> = Default::default();
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn start_input() {
    INPUT.with(|input| input.borrow_mut().clear());
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn put_input(byte: u8) {
    INPUT.with(|input| input.borrow_mut().push(byte));
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn get_output() -> u8 {
    OUTPUT.with(|output| output.borrow_mut().next().unwrap_or(0))
}

/// Set the responder function
///
/// The function must take a `&[u8]` and return a `Vec<u8>`.
#[macro_export]
macro_rules! server {
    ($respond:expr) => {
        #[no_mangle]
        pub extern "C" fn start_output() -> u64 {
            let input = $crate::INPUT.with(|i| ::std::mem::take(&mut *i.borrow_mut()));
            $crate::OUTPUT.with(|o| {
                let resp: Vec<u8> = ($respond)(input.as_slice());
                let len = resp.len();
                *o.borrow_mut() = resp.into_iter();
                len as u64
            })
        }
    };
}
