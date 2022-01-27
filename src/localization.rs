magic_statics_mod! {
	static ref RE_LOCAL_WORD_BOUNDARY: crate::Regex = crate::Regex::new(r#"\bl"#).unwrap();
}

pub fn is_localization(src: &[u8], start: usize) -> bool {
	if start == 0 {
		return false;
	}
	let mut cursor = start - 1;
	while cursor > 0 && src[cursor] == ' ' as u8 {
		cursor -= 1;
	}
	if cursor < b"local".len() - 1 {
		return false;
	}
	if &src[cursor - (b"local".len() - 1)..=cursor] != b"local" {
		return false;
	}
	if cursor <= 4 {
		return true;
	}
	RE_LOCAL_WORD_BOUNDARY.is_match(&src[cursor - (b"local".len() - 1) - 1..][..2]).unwrap()
}
