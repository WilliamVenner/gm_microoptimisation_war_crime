use crate::strip::StripTree;

magic_statics_mod! {
	static ref RE_SERVER: crate::Regex = crate::Regex::new(r#"\bSERVER\b"#).unwrap();
	static ref RE_CLIENT: crate::Regex = crate::Regex::new(r#"\bCLIENT\b"#).unwrap();
}

#[inline]
fn replace<'r, 't>(
	iter: pcre2::bytes::Matches<'r, 't>,
	strip_tree: &StripTree,
	src: &mut [u8],
	replacement: &'static [u8],
) {
	iter.map(|m| m.unwrap())
		.map(|m| m.start()..m.end())
		.filter(|m| !strip_tree.contains(m.clone()))
		.for_each(|m| src[m].copy_from_slice(replacement))
}

pub(crate) fn optimise_server(src: &mut [u8], strip_tree: &StripTree) {
	let copy = src.to_vec();
	replace(RE_SERVER.find_iter(&copy), strip_tree, src, b"true  ");
	replace(RE_CLIENT.find_iter(&copy), strip_tree, src, b"false ");
}

pub(crate) fn optimise_client(src: &mut [u8], strip_tree: &StripTree) {
	let copy = src.to_vec();
	replace(RE_SERVER.find_iter(&copy), strip_tree, src, b"false ");
	replace(RE_CLIENT.find_iter(&copy), strip_tree, src, b"true  ");
}
