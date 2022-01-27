typedef int ( *CFunc )( void* L );

class Color;

class ILuaBase
{
public:
	// Returns the amount of values on the stack
	virtual int         Top( void ) = 0;

	// Pushes a copy of the value at iStackPos to the top of the stack
	virtual void        Push( int iStackPos ) = 0;

	// Pops iAmt values from the top of the stack
	virtual void        Pop( int iAmt = 1 ) = 0;

	// Pushes table[key] on to the stack
	// table = value at iStackPos
	// key   = value at top of the stack
	// Pops the key from the stack
	virtual void        GetTable( int iStackPos ) = 0;

	// Pushes table[key] on to the stack
	// table = value at iStackPos
	// key   = strName
	virtual void        GetField( int iStackPos, const char* strName ) = 0;

	// Sets table[key] to the value at the top of the stack
	// table = value at iStackPos
	// key   = strName
	// Pops the value from the stack
	virtual void        SetField( int iStackPos, const char* strName ) = 0;

	// Creates a new table and pushes it to the top of the stack
	virtual void        CreateTable() = 0;

	// Sets table[key] to the value at the top of the stack
	// table = value at iStackPos
	// key   = value 2nd to the top of the stack
	// Pops the key and the value from the stack
	virtual void        SetTable( int iStackPos ) = 0;

	// Sets the metatable for the value at iStackPos to the value at the top of the stack
	// Pops the value off of the top of the stack
	virtual void        SetMetaTable( int iStackPos ) = 0;

	// Pushes the metatable of the value at iStackPos on to the top of the stack
	// Upon failure, returns false and does not push anything
	virtual bool        GetMetaTable( int i ) = 0;

	// Calls a function
	// To use it: Push the function on to the stack followed by each argument
	// Pops the function and arguments from the stack, leaves iResults values on the stack
	// If this function errors, any local C values will not have their destructors called!
	virtual void        Call( int iArgs, int iResults ) = 0;

	// Similar to Call
	// See: lua_pcall( void*, int, int, int )
	virtual int         PCall( int iArgs, int iResults, int iErrorFunc ) = 0;

	// Returns true if the values at iA and iB are equal
	virtual int         Equal( int iA, int iB ) = 0;

	// Returns true if the value at iA and iB are equal
	// Does not invoke metamethods
	virtual int         RawEqual( int iA, int iB ) = 0;

	// Moves the value at the top of the stack in to iStackPos
	// Any elements above iStackPos are shifted upwards
	virtual void        Insert( int iStackPos ) = 0;

	// Removes the value at iStackPos from the stack
	// Any elements above iStackPos are shifted downwards
	virtual void        Remove( int iStackPos ) = 0;

	// Allows you to iterate tables similar to pairs(...)
	// See: lua_next( void*, int );
	virtual int         Next( int iStackPos ) = 0;

protected:
		// Deprecated: Use the UserType functions instead of this
	virtual void*       NewUserdata( unsigned int iSize ) = 0;

public:
	// Throws an error and ceases execution of the function
	// If this function is called, any local C values will not have their destructors called!
	[[noreturn]]
	virtual void        ThrowError( const char* strError ) = 0;

	// Checks that the type of the value at iStackPos is iType
	// Throws and error and ceases execution of the function otherwise
	// If this function errors, any local C values will not have their destructors called!
	virtual void        CheckType( int iStackPos, int iType ) = 0;

	// Throws a pretty error message about the given argument
	// If this function is called, any local C values will not have their destructors called!
	[[noreturn]]
	virtual void        ArgError( int iArgNum, const char* strMessage ) = 0;

	// Pushes table[key] on to the stack
	// table = value at iStackPos
	// key   = value at top of the stack
	// Does not invoke metamethods
	virtual void        RawGet( int iStackPos ) = 0;

	// Sets table[key] to the value at the top of the stack
	// table = value at iStackPos
	// key   = value 2nd to the top of the stack
	// Pops the key and the value from the stack
	// Does not invoke metamethods
	virtual void        RawSet( int iStackPos ) = 0;

	// Returns the string at iStackPos. iOutLen is set to the length of the string if it is not NULL
	// If the value at iStackPos is a number, it will be converted in to a string
	// Returns NULL upon failure
	virtual const char* GetString( int iStackPos = -1, unsigned int* iOutLen = nullptr ) = 0;

	// Returns the number at iStackPos
	// Returns 0 upon failure
	virtual double      GetNumber( int iStackPos = -1 ) = 0;

	// Returns the boolean at iStackPos
	// Returns false upon failure
	virtual bool        GetBool( int iStackPos = -1 ) = 0;

	// Returns the C-Function at iStackPos
	// returns NULL upon failure
	virtual CFunc       GetCFunction( int iStackPos = -1 ) = 0;

protected:
		// Deprecated: You should probably be using the UserType functions instead of this
	virtual void*       GetUserdata( int iStackPos = -1 ) = 0;

public:
	// Pushes a nil value on to the stack
	virtual void        PushNil() = 0;

	// Pushes the given string on to the stack
	// If iLen is 0, strlen will be used to determine the string's length
	virtual void        PushString( const char* val, unsigned int iLen = 0 ) = 0;

	// Pushes the given double on to the stack
	virtual void        PushNumber( double val ) = 0;

	// Pushes the given bobolean on to the stack
	virtual void        PushBool( bool val ) = 0;

	// Pushes the given C-Function on to the stack
	virtual void        PushCFunction( CFunc val ) = 0;

	// Pushes the given C-Function on to the stack with upvalues
	// See: GetUpvalueIndex()
	virtual void        PushCClosure( CFunc val, int iVars ) = 0;


protected:
		// Deprecated: Don't use light userdata in GMod
	virtual void        PushUserdata( void* ) = 0;

public:
	// Allows for values to be stored by reference for later use
	// Make sure you call ReferenceFree when you are done with a reference
	virtual int         ReferenceCreate() = 0;
	virtual void        ReferenceFree( int i ) = 0;
	virtual void        ReferencePush( int i ) = 0;

	// Push a special value onto the top of the stack (see SPECIAL_* enums)
	virtual void        PushSpecial( int iType ) = 0;

	// Returns true if the value at iStackPos is of type iType
	// See: Types.h
	virtual bool        IsType( int iStackPos, int iType ) = 0;

	// Returns the type of the value at iStackPos
	// See: Types.h
	virtual int         GetType( int iStackPos ) = 0;

	// Returns the name associated with the given type ID
	// See: Types.h
	// Note: GetTypeName does not work with user-created types
	virtual const char* GetTypeName( int iType ) = 0;

protected:
		// Deprecated: Use CreateMetaTable
	virtual void        CreateMetaTableType( const char* strName, int iType ) = 0;

public:
	// Like Get* but throws errors and returns if they're not of the expected type
	// If these functions error, any local C values will not have their destructors called!
	virtual const char* CheckString( int iStackPos = -1 ) = 0;
	virtual double      CheckNumber( int iStackPos = -1 ) = 0;

	// Returns the length of the object at iStackPos
	// Works for: strings, tables, userdata
	virtual int         ObjLen( int iStackPos = -1 ) = 0;

	// Returns the angle at iStackPos
	virtual const void * GetAngle( int iStackPos = -1 ) = 0;

	// Returns the vector at iStackPos
	virtual const void * GetVector( int iStackPos = -1 ) = 0;

	// Pushes the given angle to the top of the stack
	virtual void        PushAngle( const void * val ) = 0;

	// Pushes the given vector to the top of the stack
	virtual void        PushVector( const void * val ) = 0;

	// Sets the void to be used by the ILuaBase implementation
	// You don't need to use this if you use the LUA_FUNCTION macro
	virtual void        SetState( void* L ) = 0;

	// Pushes the metatable associated with the given type name
	// Returns the type ID to use for this type
	// If the type doesn't currently exist, it will be created
	virtual int         CreateMetaTable( const char* strName ) = 0;

	// Pushes the metatable associated with the given type
	virtual bool        PushMetaTable( int iType ) = 0;

	// Creates a new UserData of type iType that references the given data
	virtual void        PushUserType( void* data, int iType ) = 0;

	// Sets the data pointer of the UserType at iStackPos
	// You can use this to invalidate a UserType by passing NULL
	virtual void        SetUserType( int iStackPos, void* data ) = 0;
};

class ILuaInterface : public ILuaBase
{
public:
	virtual bool Init( void *, bool ) = 0;
	virtual void Shutdown( ) = 0;
	virtual void Cycle( ) = 0;
	virtual void *Global( ) = 0;
	virtual void *GetObject( int index ) = 0;
	virtual void PushLuaObject( void *obj ) = 0;
	virtual void PushLuaFunction( CFunc func ) = 0;
	virtual void LuaError( const char *err, int index ) = 0;
	virtual void TypeError( const char *name, int index ) = 0;
	virtual void CallInternal( int args, int rets ) = 0;
	virtual void CallInternalNoReturns( int args ) = 0;
	virtual bool CallInternalGetBool( int args ) = 0;
	virtual const char *CallInternalGetString( int args ) = 0;
	virtual bool CallInternalGet( int args, void *obj ) = 0;
	virtual void NewGlobalTable( const char *name ) = 0;
	virtual void *NewTemporaryObject( ) = 0;
	virtual bool isUserData( int index ) = 0;
	virtual void *GetMetaTableObject( const char *name, int type ) = 0;
	virtual void *GetMetaTableObject( int index ) = 0;
	virtual void *GetReturn( int index ) = 0;
	virtual bool IsServer( ) = 0;
	virtual bool IsClient( ) = 0;
	virtual bool IsMenu( ) = 0;
	virtual void DestroyObject( void *obj ) = 0;
	virtual void *CreateObject( ) = 0;
	virtual void SetMember( void *table, void *key, void *value ) = 0;
	virtual void GetNewTable( ) = 0;
	virtual void SetMember( void *table, float key ) = 0;
	virtual void SetMember( void *table, float key, void *value ) = 0;
	virtual void SetMember( void *table, const char *key ) = 0;
	virtual void SetMember( void *table, const char *key, void *value ) = 0;
	virtual void SetType( unsigned char ) = 0;
	virtual void PushLong( long num ) = 0;
	virtual int GetFlags( int index ) = 0;
	virtual bool FindOnObjectsMetaTable( int objIndex, int keyIndex ) = 0;
	virtual bool FindObjectOnTable( int tableIndex, int keyIndex ) = 0;
	virtual void SetMemberFast( void *table, int keyIndex, int valueIndex ) = 0;
	virtual bool RunString( const char *filename, const char *path, const char *stringToRun, bool run, bool showErrors ) = 0;
	virtual bool IsEqual( void *objA, void *objB ) = 0;
	virtual void Error( const char *err ) = 0;
	virtual const char *GetStringOrError( int index ) = 0;
	virtual bool RunLuaModule( const char *name ) = 0;
	virtual bool FindAndRunScript( const char *filename, bool run, bool showErrors, const char *stringToRun, bool noReturns ) = 0;
	virtual void SetPathID( const char *pathID ) = 0;
	virtual const char *GetPathID( ) = 0;
	virtual void ErrorNoHalt( const char *fmt, ... ) = 0;
	virtual void Msg( const char *fmt, ... ) = 0;
	virtual void PushPath( const char *path ) = 0;
	virtual void PopPath( ) = 0;
	virtual const char *GetPath( ) = 0;
	virtual int GetColor( int index ) = 0;
	virtual void PushColor( Color color ) = 0;
	virtual int GetStack( int level, void *dbg ) = 0;
	virtual int GetInfo( const char *what, void *dbg ) = 0;
	virtual const char *GetLocal( void *dbg, int n ) = 0;
	virtual const char *GetUpvalue( int funcIndex, int n ) = 0;
	virtual bool RunStringEx( const char *filename, const char *path, const char *stringToRun, bool run, bool printErrors, bool dontPushErrors, bool noReturns ) = 0;
	virtual size_t GetDataString( int index, const char **str ) = 0;
	virtual void ErrorFromLua( const char *fmt, ... ) = 0;
	virtual const char *GetCurrentLocation( ) = 0;
	virtual void MsgColour( const void *col, const char *fmt, ... ) = 0;
	virtual void GetCurrentFile( void *outStr ) = 0;
	virtual void CompileString( void *dumper, const void *stringToCompile ) = 0;
	virtual bool CallFunctionProtected( int, int, bool ) = 0;
	virtual void Require( const char *name ) = 0;
	virtual const char *GetActualTypeName( int type ) = 0;
	virtual void PreCreateTable( int arrelems, int nonarrelems ) = 0;
	virtual void PushPooledString( int index ) = 0;
	virtual const char *GetPooledString( int index ) = 0;
	virtual void *AddThreadedCall( void * ) = 0;
	virtual void AppendStackTrace( char *, unsigned long ) = 0;
	virtual void *CreateConVar( const char *, const char *, const char *, int ) = 0;
	virtual void *CreateConCommand( const char *, const char *, int, void ( * )( const void * ), int ( * )( const char *, char ( * )[128] ) ) = 0;
};