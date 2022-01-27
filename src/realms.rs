
use crate::{strip::StripTree, cxx, localization};

magic_statics_mod! {
	static ref RE_SERVER: crate::Regex = crate::Regex::new(r#"\bSERVER\b"#).unwrap();
	static ref RE_CLIENT: crate::Regex = crate::Regex::new(r#"\bCLIENT\b"#).unwrap();
}

static mut REALM: Realm = Realm::Unknown;

#[inline]
pub fn realm() -> Realm {
	unsafe { REALM }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Realm {
	Unknown,
	Server,
	Client
}

#[inline]
fn replace<'r, 't>(
	iter: pcre2::bytes::Matches<'r, 't>,
	strip_tree: &StripTree,
	copy: &[u8],
	src: &mut [u8],
	replacement: &'static [u8],
) {
	iter.map(|m| m.unwrap())
		.map(|m| m.start()..m.end())
		.filter(|m| !strip_tree.contains(m.clone()))
		.filter(|m| !localization::is_localization(copy, m.start))
		.for_each(|m| src[m].copy_from_slice(replacement))
}

pub(crate) fn optimise_server(src: &mut [u8], strip_tree: &StripTree) {
	let copy = src.to_vec();
	replace(RE_SERVER.find_iter(&copy), strip_tree, &copy, src, b"true  ");
	replace(RE_CLIENT.find_iter(&copy), strip_tree, &copy, src, b"false ");
}

pub(crate) fn optimise_client(src: &mut [u8], strip_tree: &StripTree) {
	let copy = src.to_vec();
	replace(RE_SERVER.find_iter(&copy), strip_tree, &copy, src, b"false ");
	replace(RE_CLIENT.find_iter(&copy), strip_tree, &copy, src, b"true  ");
}

pub(crate) fn optimise(realm: Realm, src: &mut [u8], strip_tree: &StripTree) {
	match realm {
		Realm::Server => optimise_server(src, &strip_tree),
		Realm::Client => optimise_client(src, &strip_tree),
		_ => unreachable!()
	}
}

pub(super) unsafe fn detect(lua: *mut std::ffi::c_void) {
	if cxx::bridge::is_server(lua as usize) {
		REALM = Realm::Server;
	} else if cxx::bridge::is_client(lua as usize) {
		REALM = Realm::Client;
	} else {
		panic!("Couldn't detect whether this is a server or a client");
	}
}