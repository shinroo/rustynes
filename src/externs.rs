use std::os::raw::*;
use std::ffi::CString;
use std::mem;

const EMSCRIPTEN_FETCH_LOAD_TO_MEMORY: u32 = 1;
const EMSCRIPTEN_FETCH_SYNCHRONOUS: u32 = 64;

pub struct emscripten_fetch_attr_t {
    pub requestMethod: [c_char; 32],
    pub userData: *mut c_void,
    pub onsuccess: c_int,
    pub onerror: Option<unsafe extern "C" fn(_: *mut emscripten_fetch_t)>,
    pub onprogress: Option<unsafe extern "C" fn(_: *mut emscripten_fetch_t)>,
    pub attributes: u32,
    pub timeoutMSecs: c_ulong,
    pub withCredentials: c_int,
    pub destinationPath: *const c_char,
    pub userName: *const c_char,
    pub password: *const c_char,
    pub requestHeaders: *const *const c_char,
    pub overriddenMimeType: *const c_char,
    pub requestData: *const c_char,
    pub requestDataSize: usize,
}

pub struct emscripten_fetch_t {
    pub id: c_uint,
    pub userData: *mut c_void,
    pub url: *const c_char,
    pub data: *const c_char,
    pub numBytes: u64,
    pub dataOffset: u64,
    pub totalBytes: u64,
    pub readyState: c_ushort,
    pub status: c_ushort,
    pub statusText: [c_char; 64],
    pub __proxyState: u32,
    pub __attributes: emscripten_fetch_attr_t,
}

pub fn fetch(url: &str) -> *mut emscripten_fetch_t {
    let url = CString::new(url).unwrap();
    let ptr = url.as_ptr();
    unsafe {
        let mut attr: emscripten_fetch_attr_t = mem::uninitialized();
        let attr = &mut attr as *mut emscripten_fetch_attr_t;
        emscripten_fetch_attr_init(attr);
        println!("{:?}", attr);
        (*attr).attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY; // | EMSCRIPTEN_FETCH_SYNCHRONOUS;
        (*attr).onsuccess = 17;
        let b: *mut emscripten_fetch_t = emscripten_fetch(attr, ptr);
        println!("{:?}", (*b).status);
        b
    }
}

extern "C" fn onsuccess(d: *mut emscripten_fetch_t) {
    println!("{}", "aasd");
}

// pub fn now() -> i32 {
//     unsafe { ffi::emscripten_get_now() }
// }

extern "C" {
    pub fn emscripten_fetch(fetch_attr: *mut emscripten_fetch_attr_t,
                            url: *const c_char)
                            -> *mut emscripten_fetch_t;
    pub fn emscripten_fetch_attr_init(fetch_attr: *mut emscripten_fetch_attr_t);

}
