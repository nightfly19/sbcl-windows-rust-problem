//extern int initialize_lisp(int argc, const char *argv[], char *envp[];)
//       int initialize_lisp(int argc, char *argv[], char *envp[])

use std::ffi::{CString, c_char};

#[export_name = "moocow"]
pub static mut MOOCOW: Option<fn() -> i64> = None;

fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
    let args = [
        CString::new("sbcl").expect(""),
        CString::new("--core").expect(""),
        CString::new("lisp.lib").expect(""),
        CString::new("--disable-signal-handlers").expect(""),
        CString::new("--no-sysinit").expect(""),
        CString::new("--no-userinit").expect(""),
        //CString::new("--disable-debugger").expect(""),
    ];
    println!("{:?}", args);
    let mut args_c: [*const i8;8] = [std::ptr::null(); 8];
    args_c[0] = args[0].as_ptr();
    args_c[1] = args[1].as_ptr();
    args_c[2] = args[2].as_ptr();
    args_c[3] = args[3].as_ptr();
    args_c[4] = args[4].as_ptr();
    args_c[5] = args[5].as_ptr();
    unsafe {
        let lib = libloading::Library::new("sbcl\\libsbcl.dll")?;
        let func: libloading::Symbol<unsafe extern fn(argc: i32, argv: *const *const i8, envp: *const *const i8 ) -> u32> = lib.get(b"initialize_lisp")?;
        println!("Moocow: {:?}", MOOCOW);
        println!("Initing lisp");
        func(args.len() as i32, args_c.as_ptr(), std::ptr::null());
        println!("Inited lisp");
        println!("Moocow: {:?}", MOOCOW);
        let val = MOOCOW.unwrap()();
        println!("Cow result: {}", val);
        Ok(0)
    }
}

fn main() {
    call_dynamic().unwrap();
    //println!("Result: {}", );
}
