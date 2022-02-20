use std::ffi::CString;


mod wsamedge_hostfunctionexample {
    use std::os::raw::c_char;

    #[link(wasm_import_module ="host_function_example")]
    extern "C" {
        pub fn host_function_example_set_class_id(cid: u32);
        pub fn host_function_example_add_student(stu: * const c_char,len: u32) -> u64;
        pub fn host_function_example_set_class_name(clsName: *const c_char,len: u32);
        pub fn host_function_example_print();
    }

}

pub fn set_class_id(cid :u32){
    unsafe {
        wsamedge_hostfunctionexample::host_function_example_set_class_id(cid);
    }
}
pub fn set_class_name<S: AsRef<str>>(name: S){
    let name = CString::new((name.as_ref()).as_bytes()).expect("");
    unsafe {
        wsamedge_hostfunctionexample::
            host_function_example_set_class_name(
            name.as_ptr(),
            name.as_bytes().len() as u32,
        );
    }
}

pub fn add_student<S: AsRef<str>>(stu: S) -> u64 {
    let stu = CString::new((stu.as_ref()).as_bytes()).expect("");
    let stu_size: u64;
    unsafe {
       stu_size = wsamedge_hostfunctionexample::
        host_function_example_add_student(
            stu.as_ptr(),
            stu.as_bytes().len() as u32,
        );
        stu_size
    }

}

pub fn print(){
    unsafe {
        wsamedge_hostfunctionexample::host_function_example_print();
    }
}
