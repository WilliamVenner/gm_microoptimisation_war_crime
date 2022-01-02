# Wat

This attractively named repository contains a Garry's Mod module that performs a micro optimisation on all Lua scripts that make use of LuaJIT's constant folding.

The module intercepts all loading of Lua scripts and preprocesses the Lua code to replace all instances of `SERVER` and `CLIENT` with `true` and `false` depending on the current realm.

# Explanation

In Garry's Mod there are two runtime realms, the server realm and the client realm. Because Lua doesn't have conditional compilation, developers have to use an if expression to determine whether the script is running on the server or the client:

```lua
if SERVER then
	print("Hello, server!")
end
if CLIENT then
	print("Hello, client!")
end
```

The equivalent C preprocessor code would look like this:

```c
#ifdef SERVER
	printf("Hello, server!\n")
#endif
#ifdef CLIENT
	printf("Hello, client!\n")
#endif
```

# Why even bother

It's just a dumb experiment. Only the one true God Mike Pall knows how effective this might be.