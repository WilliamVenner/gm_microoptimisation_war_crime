#[cxx::bridge] pub mod bridge {
	extern "C++" {
		include!("plugin.hpp");
		unsafe fn is_client(lua: usize) -> bool;
		unsafe fn is_server(lua: usize) -> bool;
		unsafe fn CreateInterface() -> usize;
	}
}