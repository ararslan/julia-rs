extern crate libc;

type jl_value_t = *mut libc::c_void;

#[link(name = "julia")]
extern {
    // Initialization functions
    fn jl_init(julia_home_dir: *const libc::c_char);
    fn jl_is_initialized() -> libc::c_int;
    fn jl_atexit_hook(status: libc::c_int);
    fn jl_exit(status: libc::c_int);

    // Evaluation
    fn jl_eval_string(s: *const libc::c_char) -> *mut jl_value_t;

    // Version information
    fn jl_ver_major() -> libc::c_int;
    fn jl_ver_minor() -> libc::c_int;
    fn jl_ver_patch() -> libc::c_int;
    fn jl_ver_is_release() -> libc::c_int;
    fn jl_ver_string() -> *const libc::c_char;
    // FIXME: These cause segfaults
    // fn jl_git_branch() -> *const libc::c_char;
    // fn jl_git_commit() -> *const libc::c_char;

    // Exception handling
    fn jl_exception_occurred() -> jl_value_t;
    fn jl_exception_clear();
    fn jl_throw(e: jl_value_t);
    fn jl_rethrow();
    fn jl_rethrow_other(e: jl_value_t);
}

macro_rules! check_err {
    () => {{
        unsafe {
            let exoccur = jl_exception_occurred();
            if !exoccur.is_null() {
                println!("{}", "Error occurred"); // TODO: Actual error text
            }
            jl_exception_clear();
        }
    }};
}

pub struct Julia;

impl Julia {
    pub fn new(julia_dir: &str) -> Julia {
        let dir = std::ffi::CString::new(julia_dir).unwrap();
        let dir_ptr = dir.as_ptr();
        unsafe {
            jl_init(dir_ptr);
            assert!(jl_is_initialized() == 1, "Failed to initialize Julia")
        }
        Julia {}
    }

    pub fn eval_string(&self, code: &str) -> jl_value_t {
        let code_c = std::ffi::CString::new(code).unwrap();
        let code_ptr = code_c.as_ptr();
        let res = unsafe { jl_eval_string(code_ptr) };
        check_err!();
        res
    }
}

impl Drop for Julia {
    fn drop(&mut self) {
        unsafe {
            jl_atexit_hook(0);
        }
    }
}
