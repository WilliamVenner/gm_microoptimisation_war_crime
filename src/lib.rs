#![feature(c_unwind)]

#[macro_use] extern crate gmod;
#[macro_use] extern crate magic_static;

mod hax;
mod strip;
mod enums;
mod realms;

pub(crate) type Regex = pcre2::bytes::Regex;

#[magic_static::main(
	mod strip,
	mod realms
)]
pub fn init() {}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	realms::detect(lua);

	init();
	hax::init();

	0
}

#[cfg(any(debug_assertions, test))]
mod tests;
