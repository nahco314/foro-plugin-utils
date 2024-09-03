extern crate alloc;

use core::alloc::Layout;

#[allow(unused)]
fn to_array_result(arr: &[u8]) -> *mut u8 {
    let mut new_vec = (arr.len() as u64).to_le_bytes().to_vec();

    new_vec.append(&mut arr.to_vec());

    let ptr = new_vec.as_mut_ptr();
    std::mem::forget(new_vec);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn of_malloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn of_free(ptr: *mut u8, size: u32, alignment: u32) {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}

#[macro_export]
macro_rules! main_from {
    ( $f:ident ) => {
        #[no_mangle]
        pub extern "C" fn main(ptr: *mut u8, len: usize) -> i32 {
            let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
            let v: Value = serde_json::from_slice(slice).unwrap();

            let result = f(v);

            let b = serde_json::to_vec(&result).unwrap();
            let result = b.as_slice();

            let result = to_array_result(result);
            result as i32
        }
    };
}
