use std::{collections::BTreeSet, cell::RefCell, borrow::Cow};

use crate::{strip::StripTree, localization};

/// These are enums that have internal usage that means we cannot optimise them.
static FILTERED_ENUMS: &'static [&'static str] = &[
	"NUM_AI_CLASSES"
];

magic_statics_mod! {
	static ref RE_POTENTIAL_ENUM: crate::Regex = crate::Regex::new(r#"\b(?:[A-Z0-9]+?_)*?[A-Z]+\b"#).unwrap();
}

thread_local! {
	static LUA_ENUMS: RefCell<BTreeSet<LuaEnum<'static>>> = RefCell::new(BTreeSet::new());
}

#[derive(Debug, Clone)]
pub struct LuaEnum<'a> {
	pub key: Cow<'a, [u8]>,
	pub value: isize,
	pub value_literal: Vec<u8>
}
impl LuaEnum<'_> {
	fn new<'a, K: AsRef<[u8]>>(key: K, value: isize) -> Option<LuaEnum<'a>> {
		let key = key.as_ref();

		let value_literal = {
			let mut value_literal = value.to_string().into_bytes();

			let value_literal_pad = match key.len().checked_sub(value_literal.len()) {
				Some(value_literal_pad @ 1..) => value_literal_pad,
				_ => return None
			};

			value_literal.extend([' ' as u8].into_iter().cycle().take(value_literal_pad)); // pad it
			value_literal
		};

		Some(LuaEnum {
			value_literal,
			value,
			key: key.to_vec().into()
		})
	}
}
impl Eq for LuaEnum<'_> {}
impl PartialEq for LuaEnum<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.key == other.key
	}
}
impl PartialOrd for LuaEnum<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.key.partial_cmp(&other.key)
	}
}
impl Ord for LuaEnum<'_> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.key.cmp(&other.key)
	}
}
impl<'a> From<&'a [u8]> for LuaEnum<'a> {
    fn from(bytes: &'a [u8]) -> Self {
		LuaEnum { key: Cow::Borrowed(bytes), value: -1, value_literal: Vec::new() }
    }
}

unsafe fn get_global_keys<'a>(lua: gmod::lua::State) -> BTreeSet<LuaEnum<'a>> {
	let mut global_keys = BTreeSet::new();

	fn insert_global_key(global_keys: &mut BTreeSet<LuaEnum>, key: &[u8], value: isize) {
		if !RE_POTENTIAL_ENUM.is_match(key).unwrap() {
			return;
		}

		#[cfg(debug_assertions)]
		let insert_key = key.to_owned();

		#[cfg(not(debug_assertions))]
		let insert_key = key;

		let lua_enum = match LuaEnum::new(insert_key, value) {
			Some(lua_enum) => lua_enum,
			None => return
		};

		let _was_empty = global_keys.insert(lua_enum);
		debug_assert!(_was_empty, "{} was already in the set", String::from_utf8_lossy(key));
	}

	lua_stack_guard!(lua => {
		lua.push_globals();

		lua.push_nil();
		while lua.next(-2) != 0 {
			if lua.lua_type(-1) == gmod::lua::LUA_TNUMBER {
				// we only care about numerical enums
				lua.push_value(-1); // push a copy of the value
				let value = lua.to_integer(-1);
				lua.pop();

				lua.push_value(-2); // push a copy of the key
				if let Some(key) = lua.get_binary_string(-1) {
					insert_global_key(&mut global_keys, key, value);
				}
				lua.pop();
			}
			lua.pop();
		}

		lua.pop();
	});

	for filtered in FILTERED_ENUMS {
		global_keys.remove(&LuaEnum::from(filtered.as_bytes()));
	}

	global_keys
}

pub unsafe fn collect<F: Fn()>(lua: gmod::lua::State, func: F) {
	let global_keys = get_global_keys(lua);

	func();

	let new_global_keys = get_global_keys(lua);

	LUA_ENUMS.with(|enums| {
		enums.borrow_mut().extend(new_global_keys.difference(&global_keys).cloned());
	});
}

pub(crate) fn optimise(src: &mut [u8], strip_tree: &StripTree) {
	let copy = src.to_vec();
	LUA_ENUMS.with(|enums| {
		let enums = enums.borrow();

		RE_POTENTIAL_ENUM.find_iter(&copy)
			.map(|m| m.unwrap())
			.filter(|m| !strip_tree.contains(m.start()..m.end()))
			.filter(|m| !localization::is_localization(&copy, m.start()))
			.filter_map(|m| enums.get(&LuaEnum::from(&copy[m.start()..m.end()])).map(|lua_enum| (lua_enum, m.start()..m.end())))
			.for_each(|(lua_enum, range)| {
				src[range].copy_from_slice(&lua_enum.value_literal)
			})
	});
}

#[cfg(test)]
pub fn test_data() {
	LUA_ENUMS.with(|enums| {
		enums.borrow_mut().extend([
			LuaEnum::new(b"ACT_INVALID", -1).unwrap(),
			LuaEnum::new(b"ACT_RESET", 0).unwrap(),
			LuaEnum::new(b"ACT_IDLE", 1).unwrap(),
			LuaEnum::new(b"ACT_TRANSITION", 2).unwrap(),
			LuaEnum::new(b"ACT_COVER", 3).unwrap(),
			LuaEnum::new(b"ACT_COVER_MED", 4).unwrap(),
			LuaEnum::new(b"ACT_COVER_LOW", 5).unwrap(),
			LuaEnum::new(b"ACT_WALK", 6).unwrap(),
			LuaEnum::new(b"ACT_WALK_AIM", 7).unwrap(),
			LuaEnum::new(b"ACT_WALK_CROUCH", 8).unwrap(),
			LuaEnum::new(b"ACT_WALK_CROUCH_AIM", 9).unwrap(),
			LuaEnum::new(b"ACT_RUN", 10).unwrap(),
			LuaEnum::new(b"ACT_RUN_AIM", 11).unwrap(),
			LuaEnum::new(b"ACT_RUN_CROUCH", 12).unwrap(),
			LuaEnum::new(b"ACT_RUN_CROUCH_AIM", 13).unwrap(),
		].into_iter());
	});
}