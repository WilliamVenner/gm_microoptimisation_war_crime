#pragma once

#include "rust/cxx.h"
#include "gm_microoptimisation_war_crime/src/cxx/mod.rs.h"

#include <include/iserverplugin.h>
#include <include/iluainterface.h>
#include <cstdio>
#include <cstring>

class WarcrimePlugin : public IServerPluginCallbacks
{
public:
	WarcrimePlugin();
	~WarcrimePlugin();

	virtual bool 			Load(CreateInterfaceFn, CreateInterfaceFn) { return true; };
	virtual void			Unload(void) {};
	virtual void			Pause(void) {};
	virtual void			UnPause(void) {};
	virtual const char     *GetPluginDescription(void) { return "warcrime"; };
	virtual void			LevelInit(char const *) {};
	virtual void			ServerActivate(void *, int, int) {};
	virtual void			GameFrame(bool) {};
	virtual void			LevelShutdown(void) {};
	virtual void			ClientActive(void *) {};
	virtual void			ClientDisconnect(void *) {};
	virtual void			ClientPutInServer(void *, char const *) {};
	virtual void			SetCommandClient(int ) {};
	virtual void			ClientSettingsChanged(void *) {};
	virtual PLUGIN_RESULT 	ClientConnect(bool *, void *, const char *, const char *, char *, int ) { return PLUGIN_CONTINUE; };
	virtual PLUGIN_RESULT	ClientCommand(void *, const CCommand &) { return PLUGIN_CONTINUE; };
	virtual PLUGIN_RESULT	NetworkIDValidated(const char *, const char *) { return PLUGIN_CONTINUE; };
	virtual void 			OnQueryCvarValueFinished(QueryCvarCookie_t , void *, EQueryCvarValueStatus , const char *, const char *) {};
	virtual void			OnEdictAllocated(void *) {};
	virtual void			OnEdictFreed(const void *) {};
};

uintptr_t CreateInterface();

bool is_server(uintptr_t lua_ptr);
bool is_client(uintptr_t lua_ptr);