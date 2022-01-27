#pragma once

#include <plugin.hpp>

WarcrimePlugin::WarcrimePlugin() {}

static WarcrimePlugin* INSTANCE = new WarcrimePlugin();
uintptr_t CreateInterface() {
	return (uintptr_t) INSTANCE;
}

bool is_server(uintptr_t lua_ptr) {
	ILuaInterface* lua = (ILuaInterface*)lua_ptr;
	return lua->IsServer();
}

bool is_client(uintptr_t lua_ptr) {
	ILuaInterface* lua = (ILuaInterface*)lua_ptr;
	return lua->IsClient();
}