#[macro_export]
macro_rules! onefmt_plugin_setup {
    ( $f:ident ) => {
        pub fn to_array_result(arr: &[u8]) -> *mut u8 {
            let mut new_vec = (arr.len() as u64).to_le_bytes().to_vec();

            new_vec.append(&mut arr.to_vec());

            let ptr = new_vec.as_mut_ptr();
            std::mem::forget(new_vec);

            ptr
        }

        #[no_mangle]
        pub unsafe extern "C" fn of_malloc(size: u64, alignment: u64) -> u64 {
            extern crate alloc;

            use core::alloc::Layout;

            let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
            alloc::alloc::alloc(layout) as u64
        }

        #[no_mangle]
        pub unsafe extern "C" fn of_free(ptr: u64, size: u64, alignment: u64) {
            extern crate alloc;

            use core::alloc::Layout;

            let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
            alloc::alloc::dealloc(ptr as *mut u8, layout);
        }

        #[no_mangle]
        pub extern "C" fn main(ptr: u64, len: u64) -> u64 {
            let ptr = ptr as *mut u8;
            let len = len as usize;

            use serde_json;

            let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
            let v = serde_json::from_slice(slice).unwrap();

            let result = $f(v);

            let b = serde_json::to_vec(&result).unwrap();
            let result = b.as_slice();

            let result = to_array_result(result);

            result as u64
        }
    };
}
