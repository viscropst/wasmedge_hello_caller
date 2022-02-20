pub mod hello_caller_host {
    use std::os::raw::c_char;
    #[link(wasm_import_module ="hellocaller_host")]
    extern "C" {
       pub fn c_host_prompt(text :*const c_char,len:u16) -> u64;
    }
}

pub fn host_prompt<S:AsRef<str>>(text :S) -> u64 {
    use std::ffi::CString;
    let stu = CString::new((text.as_ref()).as_bytes()).expect("");
    let strx: u64;
    unsafe {
       strx = hello_caller_host::
        c_host_prompt(
            stu.as_ptr(),
            stu.as_bytes().len() as u16,
        );
        strx
    }

}