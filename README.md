# Wat

This attractively named repository contains a Garry's Mod module that performs a micro optimisation using LuaJIT's constant folding on all Lua scripts.

The module intercepts all loading of Lua scripts and preprocesses the Lua code to replace all instances of `SERVER` and `CLIENT` with `true` and `false` depending on the current realm.

The module does not replace these in comments or strings and does not change the size of the file or span of the tokens (by inserting extra whitespace.)

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

# Results

First I should probably note that this is just a dumb experiment and I expected it to not have much performance impact.

CPU branch predictors are pretty good these days and LuaJIT should hopefully help too to make this not so much of an issue for performance.

Additionally, it's quite rare that a runtime realm check would find itself in a hot loop or hot function, so the impact of overall server/game performance from such an optimisation is probably completely unnoticable.

## Bytecode

The generated bytecode is only slightly different after this hack. It would seem that LuaJIT does not do dead code elimination in this case. It does however eliminate the branch and replaces it with a direct jump.

### Sample Code

```lua
function helloWorld()
	if SERVER then
		print("Hello SERVER")
	end
	if CLIENT then
		print("Hello CLIENT")
	end
end
```

### Unoptimised

```x86asm
0000	FUNCF    3
0001	GGET     0   0      ; "SERVER"
0002	ISF          0
0003	JMP      1 => 0007
0004	GGET     0   1      ; "print"
0005	KSTR     2   2      ; "Hello SERVER"
0006	CALL     0   1   2
0007	GGET     0   3      ; "CLIENT"
0008	ISF          0
0009	JMP      1 => 0013
0010	GGET     0   1      ; "print"
0011	KSTR     2   4      ; "Hello CLIENT"
0012	CALL     0   1   2
0013	RET0     0   1
```

### Optimised

```x86asm
0000	FUNCF    3
0001	GGET     0   0      ; "print"
0002	KSTR     2   1      ; "Hello SERVER"
0003	CALL     0   1   2
0004	JMP      0 => 0005
0005	JMP      0 => 0009
0006	GGET     0   0      ; "print"
0007	KSTR     2   2      ; "Hello CLIENT"
0008	CALL     0   1   2
0009	RET0     0   1
```

## Benchmarks

```lua
jit.off() jit.on()
jit.flush()
print(jit.status()) print()

local seed = math.random(1, 2)

do
	local function globals()
		local low = math.huge
		local high = 0
		local total = 0
		for i = 1, 10 do
			local start = SysTime()
			for i = 1, 100000 do
				local n = 0
				if SERVER then
					n = n + seed
				end
				if CLIENT then
					n = n + seed
				end
			end
			local delta = SysTime() - start
			low = math.min(low, delta)
			high = math.max(high, delta)
			total = total + delta
		end
		print("(Global) Unoptimised")
		print("Low: " .. low * (10 ^ 6) .. "us")
		print("High: " .. high * (10 ^ 6) .. "us")
		print("Mean: " .. (total / 10) * (10 ^ 6) .. "us\n")
	end
	globals()
end

do
	local function localised()
		local SERVER = SERVER
		local CLIENT = CLIENT

		local low = math.huge
		local high = 0
		local total = 0
		for i = 1, 10 do
			local start = SysTime()
			for i = 1, 100000 do
				local n = 0
				if SERVER then
					n = n + seed
				end
				if CLIENT then
					n = n + seed
				end
			end
			local delta = SysTime() - start
			low = math.min(low, delta)
			high = math.max(high, delta)
			total = total + delta
		end
		print("(Local) Unoptimised")
		print("Low: " .. low * (10 ^ 6) .. "us")
		print("High: " .. high * (10 ^ 6) .. "us")
		print("Mean: " .. (total / 10) * (10 ^ 6) .. "us\n")
	end
	localised()
end

do
	local function localised_no_const()
		local SERVER = math.random(1, 2) == 1
		local CLIENT = not SERVER

		local low = math.huge
		local high = 0
		local total = 0
		for i = 1, 10 do
			local start = SysTime()
			for i = 1, 100000 do
				local n = 0
				if SERVER then
					n = n + seed
				end
				if CLIENT then
					n = n + seed
				end
			end
			local delta = SysTime() - start
			low = math.min(low, delta)
			high = math.max(high, delta)
			total = total + delta
		end
		print("(Local NoConst) Unoptimised")
		print("Low: " .. low * (10 ^ 6) .. "us")
		print("High: " .. high * (10 ^ 6) .. "us")
		print("Mean: " .. (total / 10) * (10 ^ 6) .. "us\n")
	end
	localised_no_const()
end

do
	local function optimised()
		local low = math.huge
		local high = 0
		local total = 0
		for i = 1, 10 do
			local start = SysTime()
			for i = 1, 100000 do
				local n = 0
				if true then
					n = n + seed
				end
				if false then
					n = n + seed
				end
			end
			local delta = SysTime() - start
			low = math.min(low, delta)
			high = math.max(high, delta)
			total = total + delta
		end
		print("Optimised")
		print("Low: " .. low * (10 ^ 6) .. "us")
		print("High: " .. high * (10 ^ 6) .. "us")
		print("Mean: " .. (total / 10) * (10 ^ 6) .. "us")
	end
	optimised()
end
```

```
true    SSE2    SSE3    SSE4.1  AMD     BMI2    fold    cse     dce     fwd     dse     narrow  loop    abc     sink   fuse

(Global) Unoptimised
Low: 77.999999575695us
High: 115.39999968591us
Mean: 85.85999994466us

(Local) Unoptimised
Low: 34.700000014709us
High: 53.599999773724us
Mean: 39.509999987786us

(Local NoConst) Unoptimised
Low: 34.700000014709us
High: 70.099999902595us
Mean: 39.699999979348us

Optimised
Low: 34.699999559962us
High: 51.699999858101us
Mean: 37.459999975908us
```

## Conclusion

It is clear that most of the overhead from `if SERVER` and `if CLIENT` runtime checks come from global lookups.

Unsurprisingly, the branching overhead is extremely small. We can see this because the `(Local) Unoptimised` average is only ~2.24us difference when compared to the `Optimised` average.

Because `SERVER` and `CLIENT` are just booleans, they will be copied into the localised variables. This could potentially enable additional optimisations since LuaJIT can see that the local value could be constants. I tested this with the `Local NoConst` benchmark but it seemed to have negligible difference, so in this case I don't think it was relevant.

In conclusion from these observations, this module can optimise runtime checking of realm when looking up from the global table but has negligible impact when those globals have been localised.

### Alternative Solution

A module that prepends common globals as locals to the top of scripts could be an alternative, and potentially even better method for this, but is definitely more error prone as Lua has a maximum upvalue amount and adding new locals to the top of a script could cause an error for this reason. However, a more advanced module that implements a real GLua lexer + parser could catch this issue and skip optimising that file.