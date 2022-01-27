fn main() {
	println!("cargo:rerun-if-changed=src/cxx/mod.rs");
	println!("cargo:rerun-if-changed=src/cxx/plugin.h");
	println!("cargo:rerun-if-changed=src/cxx/plugin.cpp");
	println!("cargo:rerun-if-changed=src/include/iserverplugin.h");
	println!("cargo:rerun-if-changed=src/include/iluainterface.h");
	println!("cargo:rerun-if-changed=build.rs");

	cxx_build::bridge("src/cxx/mod.rs")
		.file("src/cxx/plugin.cpp")
		.include("src/cxx")
		.flag_if_supported("-std=c++14")
		.compile("gm_microoptimisation_war_crime");
}