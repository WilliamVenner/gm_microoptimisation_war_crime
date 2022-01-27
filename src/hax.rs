#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]

use std::{ffi::{c_void, CStr}, os::raw::c_char, cell::RefCell};
use crate::*;

pub struct Detours {
	RunStringEx: gmod::detour::GenericDetour<RunStringEx>,
	CLuaEnums_InitLibraries: gmod::detour::GenericDetour<CLuaEnums_InitLibraries>,
	CGameLuaEnums_InitLibraries: gmod::detour::GenericDetour<CGameLuaEnums_InitLibraries>,
}

thread_local! {
	static DETOURS: RefCell<Option<Detours>> = RefCell::new(None);
}

unsafe fn get_lua_state(iluainterface: *mut c_void) -> gmod::lua::State {
	let ptr = *((iluainterface as *mut u8).add(std::mem::size_of::<usize>()) as *mut *mut c_void);
	gmod::lua::State(ptr)
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("stdcall"))]
#[type_alias(CLuaEnums_InitLibraries)]
extern "cdecl" fn CLuaEnums_InitLibraries_Detour(this: *mut c_void, lua_interface: *mut c_void) {
	unsafe {
		crate::enums::collect(
			get_lua_state(lua_interface),
			|| DETOURS.with(move |detour| unsafe {
				detour.borrow().as_ref().unwrap().CLuaEnums_InitLibraries.call(this, lua_interface);
			})
		);
	}
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("stdcall"))]
#[type_alias(CGameLuaEnums_InitLibraries)]
extern "cdecl" fn CGameLuaEnums_InitLibraries_Detour(this: *mut c_void, lua_interface: *mut c_void) {
	unsafe {
		crate::enums::collect(
			get_lua_state(lua_interface),
			|| DETOURS.with(move |detour| unsafe {
				detour.borrow().as_ref().unwrap().CGameLuaEnums_InitLibraries.call(this, lua_interface);
			})
		);
	}
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("stdcall"))]
#[type_alias(RunStringEx)]
extern "cdecl" fn RunStringEx_Detour(this: *mut c_void, path: *const c_char, unk1: *const c_char, src: *const c_char, unk2: bool, unk3: bool, unk4: bool, unk5: bool) -> usize {
	#[cfg(debug_assertions)]
	let path_str = unsafe { CStr::from_ptr(path) }.to_bytes();

	let src = unsafe { CStr::from_ptr(src) };

	let mut src = src.to_bytes().to_vec();
	let strip_tree = strip::generate(&src);
	realms::optimise(crate::realms::realm(), &mut src, &strip_tree);
	enums::optimise(&mut src, &strip_tree);
	src.push(0);

	#[cfg(debug_assertions)]
	if path_str == b"gamemodes/base/gamemode/animations.lua" {
		let _ = std::fs::write("gm_microoptimisation_war_crime.lua", &src);
	}

	let src = src.as_ptr() as *const c_char;

	#[allow(unused_unsafe)]
	DETOURS.with(move |detour| unsafe {
		detour.borrow().as_ref().unwrap().RunStringEx.call(
			this, path, unk1, src,
			unk2, unk3, unk4, unk5
		)
	})
}

pub(super) unsafe fn init() {
	DETOURS.with(|cell| {
		let mut cell = cell.borrow_mut();
		drop(cell.take());

		let (_serverdll, _serverdllpath) = open_library_srv!("server").expect("Failed to find server.dll!");

		cell.replace(Detours {
			RunStringEx: {
				let (_lib, _path) = open_library_srv!("lua_shared").expect("Failed to find lua_shared!");

				// It would be cool to do this with CLuaInterface::RunMacros but it's inlined on Windows
				// and uses C++ std::string which is probably not worth making a cxx bridge for.

				let RunStringEx: RunStringEx = find_gmod_signature!((_lib, _path) -> {

					win64_x86_64: [@SIG = "40 55 53 56 57 41 54 41 56 41 57 48 8D AC 24 ? ? ? ? 48 81 EC ? ? ? ? 48 8B 05 ? ? ? ? 48 33 C4 48 89 85 ? ? ? ? 49 8B F1 4D 8B F8 4C 8B F2 48 8B F9 4D 85 C9 0F 84"],
					win32_x86_64: [@SIG = "55 8B EC 81 EC ? ? ? ? 56 57 8B 7D 10 8B F1 85 FF 0F 84 ? ? ? ? 57 E8 ? ? ? ? 83 C4 04 83 F8 01 0F 8C ? ? ? ? 80 3F 1B 75 35 8B 06 6A 1B 68 ? ? ? ? 56 FF 90 ? ? ? ?"],

					linux64_x86_64: [@SIG = "55 48 89 E5 41 57 41 56 41 55 49 89 F5 41 54 53 48 89 FB 48 81 EC ? ? ? ? 44 89 85 ? ? ? ? 8B 45 18 44 89 8D ? ? ? ? 44 8B 75 10 89 85"],
					linux32_x86_64: [@SIG = "55 89 E5 57 56 53 81 EC ? ? ? ? 8B 45 18 8B 55 14 8B 5D 08 8B 7D 0C 89 85 ? ? ? ? 8B 45 1C 8B 75 10 89 85 ? ? ? ? 8B 45 20 89 85 ? ? ? ? 8B 45 24 89 85 ? ? ? ? 65 A1 ? ? ? ? 89 45 E4 31 C0 85 D2 0F 84 ? ? ? ? 89 14 24 89 95"],

					win32: [@SIG = "55 8B EC 8B 55 10 81 EC ? ? ? ? 56 8B F1 57 85 D2 0F 84 ? ? ? ? 8B CA 8D 79 01 8D 49 00 8A 01 41 84 C0 75 F9 2B CF 83 F9 01 0F 8C ? ? ? ? 80 3A 1B 75 35"],
					linux32: [@SIG = "55 89 E5 57 56 53 81 EC ? ? ? ? 8B 45 18 8B 55 14 8B 5D 08 8B 7D 0C 89 85 ? ? ? ? 8B 45 1C 8B 75 10 89 85 ? ? ? ? 8B 45 20 89 85 ? ? ? ? 8B 45 24 89 85 ? ? ? ? 65 A1 ? ? ? ? 89 45 E4 31 C0 85 D2 0F 84 ? ? ? ? 89 14 24 89 95 ? ? ? ?"],

				}).expect("Failed to find CLuaInterface::RunStringEx");

				let detour = gmod::detour::GenericDetour::new::<RunStringEx>(RunStringEx, RunStringEx_Detour).expect("Failed to detour CLuaInterface::RunStringEx");
				detour.enable().expect("Failed to enable CLuaInterface::RunStringEx detour");
				detour
			},

			// BUTTON_CODE_INVALID
			CLuaEnums_InitLibraries: {
				let CLuaEnums_InitLibraries: CLuaEnums_InitLibraries = find_gmod_signature!((_serverdll, _serverdllpath) -> {

					win64_x86_64: [@SIG = "48 8B C4 48 89 58 08 57 48 81 EC ? ? ? ? 0F 29 70 E8 48 8B CA 0F 29 78 D8 48 8B FA 44 0F 29 40 ? 44 0F 29 48 ? 44 0F 29 58 ? 44 0F 29 60 ? 44 0F 29 68 ? 48 8B 02 44 0F 29 74 24 ? 44 0F 29 7C 24 ? FF 90 ? ? ? ? F2 0F 10 35 ? ? ? ? 48 8D 15 ? ? ? ?"],
					win32_x86_64: [@SIG = "00"],

					linux64_x86_64: [@SIG = "00"],
					linux32_x86_64: [@SIG = "00"],

					win32: [@SIG = "00"],
					linux32: [@SIG = "00"],

				}).expect("Failed to find CLuaEnums::InitLibraries");

				let detour = gmod::detour::GenericDetour::new::<CLuaEnums_InitLibraries>(CLuaEnums_InitLibraries, CLuaEnums_InitLibraries_Detour).expect("Failed to detour CLuaEnums::InitLibraries");
				detour.enable().expect("Failed to enable CLuaEnums::InitLibraries detour");
				detour
			},

			// BUTTON_CODE_INVALID
			CGameLuaEnums_InitLibraries: {
				let CGameLuaEnums_InitLibraries: CGameLuaEnums_InitLibraries = find_gmod_signature!((_serverdll, _serverdllpath) -> {

					win64_x86_64: [@SIG = "48 8B C4 48 89 58 08 48 89 70 10 57 48 81 EC ? ? ? ? 0F 29 70 E8 48 8B CA 0F 29 78 D8 48 8B FA 44 0F 29 40 ? 44 0F 29 48 ? 44 0F 29 50 ? 44 0F 29 58 ? 44 0F 29 60 ? 48 8B 02 44 0F 29 6C 24 ? 44 0F 29 74 24 ? 44 0F 29 7C 24 ? FF 90 ? ? ? ? F2 0F 10 15 ? ? ? ?"],
					win32_x86_64: [@SIG = "00"],

					linux64_x86_64: [@SIG = "00"],
					linux32_x86_64: [@SIG = "00"],

					win32: [@SIG = "00"],
					linux32: [@SIG = "00"],

				}).expect("Failed to find CGameLuaEnums::InitLibraries");

				let detour = gmod::detour::GenericDetour::new::<CGameLuaEnums_InitLibraries>(CGameLuaEnums_InitLibraries, CGameLuaEnums_InitLibraries_Detour).expect("Failed to detour CGameLuaEnums::InitLibraries");
				detour.enable().expect("Failed to enable CGameLuaEnums::InitLibraries detour");
				detour
			},
		});
	});
}