#![feature(c_unwind)]

#[macro_use] extern crate gmod;
#[macro_use] extern crate magic_static;

mod localization;
mod realms;
mod enums;
mod strip;
mod hax;
mod cxx;

pub(crate) type Regex = pcre2::bytes::Regex;

#[magic_static::main(
	mod strip,
	mod realms,
	mod enums,
	mod localization
)]
pub fn init() {}

#[cfg(any(debug_assertions, test))]
mod tests;

#[no_mangle]
pub unsafe extern "C" fn CreateInterface() -> *mut std::ffi::c_void {
	extern "C" fn before_init(_: *mut std::ffi::c_void, lua: *mut std::ffi::c_void) {
		unsafe {
			realms::detect(lua);
		}
	}

	extern "C" fn newstate(lua: *mut std::ffi::c_void, _: *mut std::ffi::c_void) {
		unsafe {
			gmod::set_lua_state(lua);
			init();
			hax::init();
		}
	}

    gmserverplugin::init();
    gmserverplugin::newstate(newstate);
    gmserverplugin::before_init(before_init);

	cxx::bridge::CreateInterface() as *mut _
}