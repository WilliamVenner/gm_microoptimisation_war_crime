#![feature(c_unwind)]

#[macro_use] extern crate gmod;
#[macro_use] extern crate magic_static;

mod hax;
mod strip;
mod optimise;

pub(crate) type Regex = pcre2::bytes::Regex;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Realm {
	Unknown,
	Server,
	Client
}

static mut REALM: Realm = Realm::Unknown;

#[inline]
pub fn realm() -> Realm {
	unsafe { REALM }
}

#[magic_static::main(
	mod strip,
	mod optimise
)]
pub fn init() {}

pub fn optimise(realm: Realm, src: &mut [u8]) {
	#[cfg(test)]
	init();

	let strip_tree = strip::generate(src);
	match realm {
		Realm::Server => optimise::optimise_server(src, &strip_tree),
		Realm::Client => optimise::optimise_client(src, &strip_tree),
		_ => unreachable!()
	}
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	if lua.is_server() {
		REALM = Realm::Server;
	} else if lua.is_client() {
		REALM = Realm::Client;
	} else {
		panic!("Couldn't detect whether this is a server or a client");
	}

	init();
	hax::init();

	0
}

#[cfg(any(debug_assertions, test))]
mod tests;
