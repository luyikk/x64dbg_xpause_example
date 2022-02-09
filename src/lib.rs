use dbg64_plugins_sdk_sys::*;
use std::ffi::c_void;
use std::os::raw::c_char;

static mut PLUGIN_HANDLE: i32 = 0;
static NAME: &str = "Rust XPause";

macro_rules! dbs {
    ($str:expr) => {
        concat!($str, "\0").as_ptr() as *const c_char
    };
}

#[no_mangle]
pub unsafe extern "C" fn pluginit(init: *mut PLUG_INITSTRUCT) -> bool {
    (*init).pluginVersion = 1;
    (*init).sdkVersion = PLUG_SDKVERSION as i32;
    let name = (*init).pluginName.as_ptr() as *mut u8;
    name.copy_from(NAME.as_ptr(), NAME.len());
    PLUGIN_HANDLE = (*init).pluginHandle;
    _plugin_logputs(format!("{} init ok\0", NAME).as_ptr() as *const i8);
    true
}

#[no_mangle]
pub unsafe extern "C" fn plugsetup(setup_struct: *const PLUG_SETUPSTRUCT) {
    _plugin_registercallback(
        PLUGIN_HANDLE,
        CBTYPE_CB_CREATETHREAD,
        Some(cb_create_thread),
    );
    _plugin_registercommand(PLUGIN_HANDLE, dbs!("rpause"), Some(cb_x_pause), true);
    _plugin_registercallback(PLUGIN_HANDLE, CBTYPE_CB_MENUENTRY, Some(cb_menu_entry));
    _plugin_menuaddentry((*setup_struct).hMenu, 0, dbs!("RPause"));
}

unsafe extern "C" fn cb_menu_entry(_cb_type: i32, info: *mut c_void) {
    let info: *mut PLUG_CB_MENUENTRY = info.cast();
    if (*info).hEntry == 0 {
        DbgCmdExec(dbs!("rpause"));
    }
}

unsafe extern "C" fn cb_create_thread(_cb_type: i32, info: *mut c_void) {
    let info: *mut PLUG_CB_CREATETHREAD = info.cast();
    if (*info).dwThreadId as u64 == DbgValFromString(dbs!("$XPause_ThreadId")) {
        DbgCmdExec(dbs!("killthread $XPause_ThreadId"));
        _plugin_logputs(dbs!("Paused!"));
    }
}

unsafe extern "C" fn cb_x_pause(_argc: i32, _argsv: *mut *mut c_char) -> bool {
    DbgCmdExecDirect(dbs!("createthread DebugBreak"));
    DbgCmdExecDirect(dbs!("mov $XPause_ThreadId, $result"));
    true
}

#[no_mangle]
pub extern "stdcall" fn plugstop() -> bool {
    true
}
