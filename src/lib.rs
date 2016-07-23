extern crate libc;

#[link(name = "julia")]
extern {
    // Initialization functions
    fn jl_init(julia_home_dir: *const libc::c_char);
    fn jl_is_initialized() -> libc::c_int;

    // Version information
    fn jl_ver_major() -> libc::c_int;
    fn jl_ver_minor() -> libc::c_int;
    fn jl_ver_patch() -> libc::c_int;
    fn jl_ver_is_release() -> libc::c_int;
    fn jl_ver_string() -> *const libc::c_char;
    // FIXME: These cause segfaults
    // fn jl_git_branch() -> *const libc::c_char;
    // fn jl_git_commit() -> *const libc::c_char;
}

struct JuliaVersion {
    major: i32,
    minor: i32,
    patch: i32,
    // commit: String,
    is_release: bool,
}

impl JuliaVersion {
    pub fn new() -> JuliaVersion {
        let _major = unsafe { jl_ver_major() };
        let _minor = unsafe { jl_ver_minor() };
        let _patch = unsafe { jl_ver_patch() };
        let _is_release = unsafe { jl_ver_is_release() } == 1;

        JuliaVersion {
            major: _major,
            minor: _minor,
            patch: _patch,
            is_release: _is_release,
        }
    }
}
