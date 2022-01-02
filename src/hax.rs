use std::{ffi::{c_void, CStr}, os::raw::c_char, cell::RefCell};

thread_local! {
	static DETOUR: RefCell<Option<gmod::detour::GenericDetour<RunStringEx>>> = RefCell::new(None);
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("stdcall"))]
#[type_alias(RunStringEx)]
extern "cdecl" fn detour(this: *mut c_void, path: *const c_char, unk1: *const c_char, src: *const c_char, unk2: bool, unk3: bool, unk4: bool, unk5: bool) -> usize {
	let src = unsafe { CStr::from_ptr(src) };

	let mut src = src.to_bytes().to_vec();
	crate::optimise(crate::realm(), &mut src);
	src.push(0);

	let src = src.as_ptr() as *const c_char;

	DETOUR.with(move |detour| {
		detour.borrow().as_ref().unwrap().call(
			this, path, unk1, src,
			unk2, unk3, unk4, unk5
		)
	})
}

pub(super) unsafe fn init() {
	DETOUR.with(|cell| {
		let mut cell = cell.borrow_mut();
		drop(cell.take());

		let (_lib, _path) = open_library_srv!("lua_shared").expect("Failed to find lua_shared!");

		// It would be cool to do this with CLuaInterface::RunMacros but it's inlined on Windows
		// and uses C++ std::string which is probably not worth making a cxx bridge for.

		let runstringex: RunStringEx = find_gmod_signature!((_lib, _path) -> {

			win64_x86_64: [@SIG = "40 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 48 8B 05 ? ? ? ? 48 33 C4 48 89 85 ? ? ? ? 49 8B F1 4D 8B F8 4C 8B F2 48 8B F9 4D 85 C9 0F 84"],
			win32_x86_64: [@SIG = "55 8B EC 81 EC ? ? ? ? 56 57 8B 7D 10 8B F1 85 FF 0F 84 ? ? ? ? 57 E8 ? ? ? ? 83 C4 04 83 F8 01 0F 8C ? ? ? ? 80 3F 1B 75 35 8B 06 6A 1B 68 ? ? ? ? 56 FF 90 ? ? ? ?"],

			linux64_x86_64: [@SIG = "55 48 89 E5 41 57 41 56 41 55 49 89 F5 41 54 53 48 89 FB 48 81 EC ? ? ? ? 44 89 85 ? ? ? ? 8B 45 18 44 89 8D ? ? ? ? 44 8B 75 10 89 85"],
			linux32_x86_64: [@SIG = "55 89 E5 57 56 53 81 EC ? ? ? ? 8B 45 18 8B 55 14 8B 5D 08 8B 7D 0C 89 85 ? ? ? ? 8B 45 1C 8B 75 10 89 85 ? ? ? ? 8B 45 20 89 85 ? ? ? ? 8B 45 24 89 85 ? ? ? ? 65 A1 ? ? ? ? 89 45 E4 31 C0 85 D2 0F 84 ? ? ? ? 89 14 24 89 95"],

			win32: [@SIG = "55 8B EC 8B 55 10 81 EC ? ? ? ? 56 8B F1 57 85 D2 0F 84 ? ? ? ? 8B CA 8D 79 01 8D 49 00 8A 01 41 84 C0 75 F9 2B CF 83 F9 01 0F 8C ? ? ? ? 80 3A 1B 75 35"],
			linux32: [@SIG = "55 89 E5 57 56 53 81 EC ? ? ? ? 8B 45 18 8B 55 14 8B 5D 08 8B 7D 0C 89 85 ? ? ? ? 8B 45 1C 8B 75 10 89 85 ? ? ? ? 8B 45 20 89 85 ? ? ? ? 8B 45 24 89 85 ? ? ? ? 65 A1 ? ? ? ? 89 45 E4 31 C0 85 D2 0F 84 ? ? ? ? 89 14 24 89 95 ? ? ? ?"],

		}).expect("Failed to find CLuaInterface::RunStringEx");

		let detour = gmod::detour::GenericDetour::new::<RunStringEx>(runstringex, detour).expect("Failed to detour CLuaInterface::RunStringEx");
		detour.enable().expect("Failed to enable CLuaInterface::RunStringEx detour");

		cell.replace(detour);
	});
}