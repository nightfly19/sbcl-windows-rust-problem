//extern int initialize_lisp(int argc, const char *argv[], char *envp[];)
//       int initialize_lisp(int argc, char *argv[], char *envp[])

use std::ffi::{CString, c_char};

fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
    let args = [
        CString::new("sbcl").expect(""),
        CString::new("--core").expect(""),
        CString::new("C:\\Users\\sage\\hello_world\\lisp.lib").expect(""),
    ];
    println!("{:?}", args);
    let mut args_c: [*const i8;4] = [std::ptr::null(); 4];
    args_c[0] = args[0].as_ptr();
    args_c[1] = args[1].as_ptr();
    args_c[2] = args[2].as_ptr();
    //args_c.as_ptr();
    unsafe {
        let lib = libloading::Library::new("C:\\Users\\sage\\hello_world\\libsbcl.dll")?;
        let func: libloading::Symbol<unsafe extern fn(argc: i32, argv: *const *const i8, envp: *const *const i8 ) -> u32> = lib.get(b"initialize_lisp")?;
        //Ok(func(0 as i32, std::ptr::null()))
        //println!("{:?}", args_c);
        //println!("{:?}", args[0]);
        Ok(func(args_c.len() as i32, args_c.as_ptr(), std::ptr::null()))
    }
}

fn main() {
    call_dynamic().unwrap();
    //println!("Result: {}", );
}
