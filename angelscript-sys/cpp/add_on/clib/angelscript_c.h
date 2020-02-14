/*
   AngelCode Scripting Library
   Copyright (c) 2003-2017 Andreas Jonsson

   This software is provided 'as-is', without any express or implied
   warranty. In no event will the authors be held liable for any
   damages arising from the use of this software.

   Permission is granted to anyone to use this software for any
   purpose, including commercial applications, and to alter it and
   redistribute it freely, subject to the following restrictions:

   1. The origin of this software must not be misrepresented; you
      must not claim that you wrote the original software. If you use
      this software in a product, an acknowledgment in the product
      documentation would be appreciated but is not required.

   2. Altered source versions must be plainly marked as such, and
      must not be misrepresented as being the original software.

   3. This notice may not be removed or altered from any source
      distribution.

   The original version of this library can be located at:
   http://www.angelcode.com/angelscript/

   Andreas Jonsson
   andreas@angelcode.com
*/


//
// angelscript_c.h
//
// The script engine interface for the C language.
//
// The idea is that the library should be compiled with a C++ compiler with the AS_C_INTERFACE 
// preprocessor word defined. The C application will then be able to link with the library and
// use this header file to interact with it.
//
// Note: This header file is not yet complete. I'd appreciate any help with completing it.
//

#ifndef ANGELSCRIPT_C_H
#define ANGELSCRIPT_C_H

#define ANGELSCRIPT_VERSION        23102


#ifdef AS_USE_NAMESPACE
 #define BEGIN_AS_NAMESPACE namespace AngelScript {
 #define END_AS_NAMESPACE }
#else
 #define BEGIN_AS_NAMESPACE
 #define END_AS_NAMESPACE
#endif

BEGIN_AS_NAMESPACE

// Data types

typedef struct asIScriptEngine asIScriptEngine;
typedef struct asIScriptModule asIScriptModule;
typedef struct asIScriptContext asIScriptContext;
typedef struct asIScriptGeneric asIScriptGeneric;
typedef struct asIScriptObject asIScriptObject;
typedef struct asITypeInfo asITypeInfo;
typedef struct asIScriptFunction asIScriptFunction;
typedef struct asIBinaryStream asIBinaryStream;
typedef struct asIJITCompiler asIJITCompiler;
typedef struct asIThreadManager asIThreadManager;
typedef struct asILockableSharedBool asILockableSharedBool;

// Enumerations and constants

// Return codes
typedef enum 
{
	asSUCCESS                              =  0,
	asERROR                                = -1,
	asCONTEXT_ACTIVE                       = -2,
	asCONTEXT_NOT_FINISHED                 = -3,
	asCONTEXT_NOT_PREPARED                 = -4,
	asINVALID_ARG                          = -5,
	asNO_FUNCTION                          = -6,
	asNOT_SUPPORTED                        = -7,
	asINVALID_NAME                         = -8,
	asNAME_TAKEN                           = -9,
	asINVALID_DECLARATION                  = -10,
	asINVALID_OBJECT                       = -11,
	asINVALID_TYPE                         = -12,
	asALREADY_REGISTERED                   = -13,
	asMULTIPLE_FUNCTIONS                   = -14,
	asNO_MODULE                            = -15,
	asNO_GLOBAL_VAR                        = -16,
	asINVALID_CONFIGURATION                = -17,
	asINVALID_INTERFACE                    = -18,
	asCANT_BIND_ALL_FUNCTIONS              = -19,
	asLOWER_ARRAY_DIMENSION_NOT_REGISTERED = -20,
	asWRONG_CONFIG_GROUP                   = -21,
	asCONFIG_GROUP_IS_IN_USE               = -22,
	asILLEGAL_BEHAVIOUR_FOR_TYPE           = -23,
	asWRONG_CALLING_CONV                   = -24,
	asBUILD_IN_PROGRESS                    = -25,
	asINIT_GLOBAL_VARS_FAILED              = -26,
	asOUT_OF_MEMORY                        = -27,
	asMODULE_IS_IN_USE                     = -28
} asERetCodes;
// Engine properties
typedef enum 
{
	asEP_ALLOW_UNSAFE_REFERENCES            = 1,
	asEP_OPTIMIZE_BYTECODE                  = 2,
	asEP_COPY_SCRIPT_SECTIONS               = 3,
	asEP_MAX_STACK_SIZE                     = 4,
	asEP_USE_CHARACTER_LITERALS             = 5,
	asEP_ALLOW_MULTILINE_STRINGS            = 6,
	asEP_ALLOW_IMPLICIT_HANDLE_TYPES        = 7,
	asEP_BUILD_WITHOUT_LINE_CUES            = 8,
	asEP_INIT_GLOBAL_VARS_AFTER_BUILD       = 9,
	asEP_REQUIRE_ENUM_SCOPE                 = 10,
	asEP_SCRIPT_SCANNER                     = 11,
	asEP_INCLUDE_JIT_INSTRUCTIONS           = 12,
	asEP_STRING_ENCODING                    = 13,
	asEP_PROPERTY_ACCESSOR_MODE             = 14,
	asEP_EXPAND_DEF_ARRAY_TO_TMPL           = 15,
	asEP_AUTO_GARBAGE_COLLECT               = 16,
	asEP_DISALLOW_GLOBAL_VARS               = 17,
	asEP_ALWAYS_IMPL_DEFAULT_CONSTRUCT      = 18,
	asEP_COMPILER_WARNINGS                  = 19,
	asEP_DISALLOW_VALUE_ASSIGN_FOR_REF_TYPE = 20,
	asEP_ALTER_SYNTAX_NAMED_ARGS            = 21,
	asEP_DISABLE_INTEGER_DIVISION           = 22,
	asEP_DISALLOW_EMPTY_LIST_ELEMENTS       = 23,
	asEP_PRIVATE_PROP_AS_PROTECTED          = 24,
	asEP_ALLOW_UNICODE_IDENTIFIERS          = 25,
	asEP_HEREDOC_TRIM_MODE                  = 26,

	asEP_LAST_PROPERTY
} asEEngineProp;

// Calling conventions
typedef enum 
{
	asCALL_CDECL             = 0,
	asCALL_STDCALL           = 1,
	asCALL_THISCALL_ASGLOBAL = 2,
	asCALL_THISCALL          = 3,
	asCALL_CDECL_OBJLAST     = 4,
	asCALL_CDECL_OBJFIRST    = 5,
	asCALL_GENERIC           = 6,
	asCALL_THISCALL_OBJLAST  = 7,
	asCALL_THISCALL_OBJFIRST = 8
} asECallConvTypes;

// Object type flags
typedef enum 
{
	asOBJ_REF                        = (1<<0),
	asOBJ_VALUE                      = (1<<1),
	asOBJ_GC                         = (1<<2),
	asOBJ_POD                        = (1<<3),
	asOBJ_NOHANDLE                   = (1<<4),
	asOBJ_SCOPED                     = (1<<5),
	asOBJ_TEMPLATE                   = (1<<6),
	asOBJ_ASHANDLE                   = (1<<7),
	asOBJ_APP_CLASS                  = (1<<8),
	asOBJ_APP_CLASS_CONSTRUCTOR      = (1<<9),
	asOBJ_APP_CLASS_DESTRUCTOR       = (1<<10),
	asOBJ_APP_CLASS_ASSIGNMENT       = (1<<11),
	asOBJ_APP_CLASS_COPY_CONSTRUCTOR = (1<<12),
	asOBJ_APP_CLASS_C                = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR),
	asOBJ_APP_CLASS_CD               = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_DESTRUCTOR),
	asOBJ_APP_CLASS_CA               = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_ASSIGNMENT),
	asOBJ_APP_CLASS_CK               = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_CDA              = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_DESTRUCTOR + asOBJ_APP_CLASS_ASSIGNMENT),
	asOBJ_APP_CLASS_CDK              = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_DESTRUCTOR + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_CAK              = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_ASSIGNMENT + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_CDAK             = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_CONSTRUCTOR + asOBJ_APP_CLASS_DESTRUCTOR + asOBJ_APP_CLASS_ASSIGNMENT + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_D                = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_DESTRUCTOR),
	asOBJ_APP_CLASS_DA               = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_DESTRUCTOR + asOBJ_APP_CLASS_ASSIGNMENT),
	asOBJ_APP_CLASS_DK               = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_DESTRUCTOR + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_DAK              = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_DESTRUCTOR + asOBJ_APP_CLASS_ASSIGNMENT + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_A                = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_ASSIGNMENT),
	asOBJ_APP_CLASS_AK               = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_ASSIGNMENT + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_CLASS_K                = (asOBJ_APP_CLASS + asOBJ_APP_CLASS_COPY_CONSTRUCTOR),
	asOBJ_APP_PRIMITIVE              = (1<<13),
	asOBJ_APP_FLOAT                  = (1<<14),
	asOBJ_APP_ARRAY                  = (1<<15),
	asOBJ_APP_CLASS_ALLINTS          = (1<<16),
	asOBJ_APP_CLASS_ALLFLOATS        = (1<<17),
	asOBJ_NOCOUNT                    = (1<<18),
	asOBJ_APP_CLASS_ALIGN8           = (1<<19),
	asOBJ_IMPLICIT_HANDLE            = (1<<20),
	asOBJ_MASK_VALID_FLAGS           = 0x1FFFFF,
	// Internal flags
	asOBJ_SCRIPT_OBJECT              = (1<<21),
	asOBJ_SHARED                     = (1<<22),
	asOBJ_NOINHERIT                  = (1<<23),
	asOBJ_FUNCDEF                    = (1<<24),
	asOBJ_LIST_PATTERN               = (1<<25),
	asOBJ_ENUM                       = (1<<26),
	asOBJ_TEMPLATE_SUBTYPE           = (1<<27),
	asOBJ_TYPEDEF                    = (1<<28),
	asOBJ_ABSTRACT                   = (1<<29),
	asOBJ_APP_ALIGN16                = (1<<30)
} asEObjTypeFlags;

// Behaviours
typedef enum 
{
	// Value object memory management
	asBEHAVE_CONSTRUCT,
	asBEHAVE_LIST_CONSTRUCT,
	asBEHAVE_DESTRUCT,

	// Reference object memory management
	asBEHAVE_FACTORY,
	asBEHAVE_LIST_FACTORY,
	asBEHAVE_ADDREF,
	asBEHAVE_RELEASE,
	asBEHAVE_GET_WEAKREF_FLAG,

	// Object operators
	asBEHAVE_TEMPLATE_CALLBACK,

	// Garbage collection behaviours
	asBEHAVE_FIRST_GC,
	 asBEHAVE_GETREFCOUNT = asBEHAVE_FIRST_GC,
	 asBEHAVE_SETGCFLAG,
	 asBEHAVE_GETGCFLAG,
	 asBEHAVE_ENUMREFS,
	 asBEHAVE_RELEASEREFS,
	asBEHAVE_LAST_GC = asBEHAVE_RELEASEREFS,

	asBEHAVE_MAX
} asEBehaviours;

// Context states
typedef enum 
{
	asEXECUTION_FINISHED      = 0,
	asEXECUTION_SUSPENDED     = 1,
	asEXECUTION_ABORTED       = 2,
	asEXECUTION_EXCEPTION     = 3,
	asEXECUTION_PREPARED      = 4,
	asEXECUTION_UNINITIALIZED = 5,
	asEXECUTION_ACTIVE        = 6,
	asEXECUTION_ERROR         = 7
} asEContextState;

// Message types
typedef enum 
{
	asMSGTYPE_ERROR       = 0,
	asMSGTYPE_WARNING     = 1,
	asMSGTYPE_INFORMATION = 2
} asEMsgType;

// Garbage collector flags
typedef enum
{
	asGC_FULL_CYCLE      = 1,
	asGC_ONE_STEP        = 2,
	asGC_DESTROY_GARBAGE = 4,
	asGC_DETECT_GARBAGE  = 8
} asEGCFlags;

// Token classes
typedef enum 
{
	asTC_UNKNOWN    = 0,
	asTC_KEYWORD    = 1,
	asTC_VALUE      = 2,
	asTC_IDENTIFIER = 3,
	asTC_COMMENT    = 4,
	asTC_WHITESPACE = 5
} asETokenClass;

// Type id flags
typedef enum 
{
	asTYPEID_VOID           = 0,
	asTYPEID_BOOL           = 1,
	asTYPEID_INT8           = 2,
	asTYPEID_INT16          = 3,
	asTYPEID_INT32          = 4,
	asTYPEID_INT64          = 5,
	asTYPEID_UINT8          = 6,
	asTYPEID_UINT16         = 7,
	asTYPEID_UINT32         = 8,
	asTYPEID_UINT64         = 9,
	asTYPEID_FLOAT          = 10,
	asTYPEID_DOUBLE         = 11,
	asTYPEID_OBJHANDLE      = 0x40000000,
	asTYPEID_HANDLETOCONST  = 0x20000000,
	asTYPEID_MASK_OBJECT    = 0x1C000000,
	asTYPEID_APPOBJECT      = 0x04000000,
	asTYPEID_SCRIPTOBJECT   = 0x08000000,
	asTYPEID_TEMPLATE       = 0x10000000,
	asTYPEID_MASK_SEQNBR    = 0x03FFFFFF
} asETypeIdFlags;

// Type modifiers
typedef enum
{
	asTM_NONE     = 0,
	asTM_INREF    = 1,
	asTM_OUTREF   = 2,
	asTM_INOUTREF = 3,
	asTM_CONST    = 4
} asETypeModifiers;

// GetModule flags
typedef enum
{
	asGM_ONLY_IF_EXISTS       = 0,
	asGM_CREATE_IF_NOT_EXISTS = 1,
	asGM_ALWAYS_CREATE        = 2
} asEGMFlags;

// Compile flags
typedef enum
{
	asCOMP_ADD_TO_MODULE = 1
} asECompileFlags;

// Function types
typedef enum
{
	asFUNC_DUMMY     =-1,
	asFUNC_SYSTEM    = 0,
	asFUNC_SCRIPT    = 1,
	asFUNC_INTERFACE = 2,
	asFUNC_VIRTUAL   = 3,
	asFUNC_FUNCDEF   = 4,
	asFUNC_IMPORTED  = 5,
	asFUNC_DELEGATE  = 6
} asEFuncType;

//
// asBYTE  =  8 bits
// asWORD  = 16 bits
// asDWORD = 32 bits
// asQWORD = 64 bits
// asPWORD = size of pointer
//
typedef unsigned char  asBYTE;
typedef unsigned short asWORD;
typedef unsigned int   asUINT;
typedef size_t         asPWORD;
#ifdef __LP64__
	typedef unsigned int  asDWORD;
	typedef unsigned long asQWORD;
	typedef long asINT64;
#else
	typedef unsigned long asDWORD;
  #if defined(__GNUC__) || defined(__MWERKS__)
	typedef unsigned long long asQWORD;
	typedef long long asINT64;
  #else
	typedef unsigned __int64 asQWORD;
	typedef __int64 asINT64;
  #endif
#endif
typedef enum { asTRUE = 1, asFALSE = 0 } asBOOL;


typedef void (*asBINARYREADFUNC_t)(void *ptr, asUINT size, void *param);
typedef void (*asBINARYWRITEFUNC_t)(const void *ptr, asUINT size, void *param);
typedef void (*asFUNCTION_t)();
typedef void (*asGENFUNC_t)(asIScriptGeneric *);
typedef void *(*asALLOCFUNC_t)(size_t);
typedef void (*asFREEFUNC_t)(void *);
typedef void (*asCLEANENGINEFUNC_t)(asIScriptEngine *);
typedef void (*asCLEANMODULEFUNC_t)(asIScriptModule *);
typedef void (*asCLEANCONTEXTFUNC_t)(asIScriptContext *);
typedef void (*asCLEANFUNCTIONFUNC_t)(asIScriptFunction *);
typedef void (*asCLEANTYPEINFOFUNC_t)(asITypeInfo *);
typedef void (*asCLEANSCRIPTOBJECTFUNC_t)(asIScriptObject *);
typedef asIScriptContext *(*asREQUESTCONTEXTFUNC_t)(asIScriptEngine *, void *);
typedef void (*asRETURNCONTEXTFUNC_t)(asIScriptEngine *, asIScriptContext *, void *);

typedef struct 
{
	const char *section;
	int         row;
	int         col;
	asEMsgType  type;
	const char *message;
} asSMessageInfo;


// API functions

// ANGELSCRIPT_EXPORT is defined when compiling the dll or lib
// ANGELSCRIPT_DLL_LIBRARY_IMPORT is defined when dynamically linking to the
// dll through the link lib automatically generated by MSVC++
// ANGELSCRIPT_DLL_MANUAL_IMPORT is defined when manually loading the dll
// Don't define anything when linking statically to the lib

#if defined(WIN32) || defined(_WIN32) || defined(__CYGWIN__)
  #if defined(ANGELSCRIPT_EXPORT)
    #define AS_API __declspec(dllexport)
  #elif defined(ANGELSCRIPT_DLL_LIBRARY_IMPORT)
    #define AS_API __declspec(dllimport)
  #else // statically linked library
    #define AS_API
  #endif
#elif defined(__GNUC__)
  #if defined(ANGELSCRIPT_EXPORT)
    #define AS_API __attribute__((visibility ("default")))
  #else
    #define AS_API
  #endif
#else
  #define AS_API
#endif

#ifndef ANGELSCRIPT_DLL_MANUAL_IMPORT
#ifdef __cplusplus
extern "C"
{
#endif
	// Engine
	AS_API asIScriptEngine *asCreateScriptEngine(asDWORD version);
	AS_API const char      *asGetLibraryVersion();
	AS_API const char      *asGetLibraryOptions();

	// Context
	AS_API asIScriptContext *asGetActiveContext();

	// Thread support
	AS_API int               asPrepareMultithread(asIThreadManager *externalMgr);
	AS_API void              asUnprepareMultithread();
	AS_API asIThreadManager *asGetThreadManager();
	AS_API void              asAcquireExclusiveLock();
	AS_API void              asReleaseExclusiveLock();
	AS_API void              asAcquireSharedLock();
	AS_API void              asReleaseSharedLock();
	AS_API int               asAtomicInc(int &value);
	AS_API int               asAtomicDec(int &value);
	AS_API int               asThreadCleanup();

	// Memory management
	AS_API int   asSetGlobalMemoryFunctions(asALLOCFUNC_t allocFunc, asFREEFUNC_t freeFunc);
	AS_API int   asResetGlobalMemoryFunctions();
	AS_API void *asAllocMem(size_t size);
	AS_API void  asFreeMem(void *mem);

	// Auxiliary
	AS_API asILockableSharedBool *asCreateLockableSharedBool();

	///////////////////////////////////////////
	// asIScriptEngine
	
	// Memory management
	AS_API int                asEngine_AddRef(asIScriptEngine *e);
	AS_API int                asEngine_Release(asIScriptEngine *e);
	AS_API int                asEngine_ShutDownAndRelease(asIScriptEngine *e);

	// Engine properties
	AS_API int                asEngine_SetEngineProperty(asIScriptEngine *e, asEEngineProp property, asPWORD value);
	AS_API asPWORD            asEngine_GetEngineProperty(asIScriptEngine *e, asEEngineProp property);

	// Compiler messages
	AS_API int                asEngine_SetMessageCallback(asIScriptEngine *e, asFUNCTION_t callback, void *obj, asDWORD callConv);
	AS_API int                asEngine_ClearMessageCallback(asIScriptEngine *e);
	AS_API int                asEngine_WriteMessage(asIScriptEngine *e, const char *section, int row, int col, asEMsgType type, const char *message);

	// JIT Compiler
	AS_API int                asEngine_SetJITCompiler(asIScriptEngine *e, asIJITCompiler *compiler);
	AS_API asIJITCompiler *   asEngine_GetJITCompiler(asIScriptEngine *e);

	// Global functions
	AS_API int                asEngine_RegisterGlobalFunction(asIScriptEngine *e, const char *declaration, asFUNCTION_t funcPointer, asDWORD callConv, void *auxiliary);
	AS_API asUINT             asEngine_GetGlobalFunctionCount(asIScriptEngine *e);
	AS_API asIScriptFunction* asEngine_GetGlobalFunctionByIndex(asIScriptEngine *e, asUINT index);
	AS_API asIScriptFunction* asEngine_GetGlobalFunctionByDecl(asIScriptEngine *e, const char *declaration);

	// Global properties
	AS_API int                asEngine_RegisterGlobalProperty(asIScriptEngine *e, const char *declaration, void *pointer);
	AS_API asUINT             asEngine_GetGlobalPropertyCount(asIScriptEngine *e);
	AS_API int                asEngine_GetGlobalPropertyByIndex(asIScriptEngine *e, asUINT index, const char **name, const char **nameSpace, int *typeId, asBOOL *isConst, const char **configGroup, void **pointer, asDWORD *accessMask);
	AS_API int                asEngine_GetGlobalPropertyIndexByName(asIScriptEngine *e, const char *name);
	AS_API int                asEngine_GetGlobalPropertyIndexByDecl(asIScriptEngine *e, const char *decl);

	// Object types
	AS_API int                asEngine_RegisterObjectType(asIScriptEngine *e, const char *name, int byteSize, asDWORD flags);
	AS_API int                asEngine_RegisterObjectProperty(asIScriptEngine *e, const char *obj, const char *declaration, int byteOffset);
	AS_API int                asEngine_RegisterObjectMethod(asIScriptEngine *e, const char *obj, const char *declaration, asFUNCTION_t funcPointer, asDWORD callConv, void *auxiliary);
	AS_API int                asEngine_RegisterObjectBehaviour(asIScriptEngine *e, const char *datatype, asEBehaviours behaviour, const char *declaration, asFUNCTION_t funcPointer, asDWORD callConv, void *auxiliary);
	AS_API int                asEngine_RegisterInterface(asIScriptEngine *e, const char *name);
	AS_API int                asEngine_RegisterInterfaceMethod(asIScriptEngine *e, const char *intf, const char *declaration);
	AS_API asUINT             asEngine_GetObjectTypeCount(asIScriptEngine *e);
	AS_API asITypeInfo *      asEngine_GetObjectTypeByIndex(asIScriptEngine *e, asUINT index);

	// String factory
	AS_API int                asEngine_RegisterStringFactory(asIScriptEngine *e, const char *datatype, asFUNCTION_t factoryFunc, asDWORD callConv, void *auxiliary);
	AS_API int                asEngine_GetStringFactoryReturnTypeId(asIScriptEngine *e, asDWORD *flags);

	// Default array type
	AS_API int                asEngine_RegisterDefaultArrayType(asIScriptEngine *e, const char *type);
	AS_API int                asEngine_GetDefaultArrayTypeId(asIScriptEngine *e);

	// Enums
	AS_API int                asEngine_RegisterEnum(asIScriptEngine *e, const char *type);
	AS_API int                asEngine_RegisterEnumValue(asIScriptEngine *e, const char *type, const char *name, int value);
	AS_API asUINT             asEngine_GetEnumCount(asIScriptEngine *e);
	AS_API asITypeInfo *      asEngine_GetEnumByIndex(asIScriptEngine *e, asUINT index);

	// Funcdefs
	AS_API int                asEngine_RegisterFuncdef(asIScriptEngine *e, const char *decl);
	AS_API asUINT             asEngine_GetFuncdefCount(asIScriptEngine *e);
	AS_API asITypeInfo *      asEngine_GetFuncdefByIndex(asIScriptEngine *e, asUINT index);

	// Typedefs
	AS_API int                asEngine_RegisterTypedef(asIScriptEngine *e, const char *type, const char *decl);
	AS_API asUINT             asEngine_GetTypedefCount(asIScriptEngine *e);
	AS_API asITypeInfo *      asEngine_GetTypedefByIndex(asIScriptEngine *e, asUINT index);

	// Configuration groups
	AS_API int                asEngine_BeginConfigGroup(asIScriptEngine *e, const char *groupName);
	AS_API int                asEngine_EndConfigGroup(asIScriptEngine *e);
	AS_API int                asEngine_RemoveConfigGroup(asIScriptEngine *e, const char *groupName);
	AS_API asDWORD            asEngine_SetDefaultAccessMask(asIScriptEngine *e, asDWORD defaultMask);
	AS_API int                asEngine_SetDefaultNamespace(asIScriptEngine *e, const char *nameSpace);
	AS_API const char *       asEngine_GetDefaultNamespace(asIScriptEngine *e);

	// Script modules
	AS_API asIScriptModule *  asEngine_GetModule(asIScriptEngine *e, const char *module, asEGMFlags flag);
	AS_API int                asEngine_DiscardModule(asIScriptEngine *e, const char *module);
	AS_API asUINT             asEngine_GetModuleCount(asIScriptEngine *e);
	AS_API asIScriptModule *  asEngine_GetModuleByIndex(asIScriptEngine *e, asUINT index);

	// Script functions
	AS_API asIScriptFunction *asEngine_GetFunctionById(asIScriptEngine *e, int funcId);

	// Type identification
	AS_API int                asEngine_GetTypeIdByDecl(asIScriptEngine *e, const char *decl);
	AS_API const char *       asEngine_GetTypeDeclaration(asIScriptEngine *e, int typeId, asBOOL includeNamespace);
	AS_API int                asEngine_GetSizeOfPrimitiveType(asIScriptEngine *e, int typeId);
	AS_API asITypeInfo   *asEngine_GetTypeInfoById(asIScriptEngine *e, int typeId);
	AS_API asITypeInfo   *asEngine_GetTypeInfoByName(asIScriptEngine *e, const char *name);
	AS_API asITypeInfo   *asEngine_GetTypeInfoByDecl(asIScriptEngine *e, const char *decl);

	// Script execution
	AS_API asIScriptContext *     asEngine_CreateContext(asIScriptEngine *e);
	AS_API void *                 asEngine_CreateScriptObject(asIScriptEngine *e, asITypeInfo *type);
	AS_API void *                 asEngine_CreateScriptObjectCopy(asIScriptEngine *e, void *obj, asITypeInfo *type);
	AS_API void *                 asEngine_CreateUninitializedScriptObject(asIScriptEngine *e, asITypeInfo *type);
	AS_API asIScriptFunction *    asEngine_CreateDelegate(asIScriptEngine *e, asIScriptFunction *func, void *obj);
	AS_API int                    asEngine_AssignScriptObject(asIScriptEngine *e, void *dstObj, void *srcObj, asITypeInfo *type);
	AS_API void                   asEngine_ReleaseScriptObject(asIScriptEngine *e, void *obj, asITypeInfo *type);
	AS_API void                   asEngine_AddRefScriptObject(asIScriptEngine *e, void *obj, asITypeInfo *type);
	AS_API int                    asEngine_RefCastObject(asIScriptEngine *e, void *obj, asITypeInfo *fromType, asITypeInfo *toType, void **newPtr, bool useOnlyImplicitCast);
	AS_API asILockableSharedBool *asEngine_GetWeakRefFlagOfScriptObject(asIScriptEngine *e, void *obj, asITypeInfo *type);

	// Context pooling
	AS_API asIScriptContext      *asEngine_RequestContext(asIScriptEngine *e);
	AS_API void                   asEngine_ReturnContext(asIScriptEngine *e, asIScriptContext *ctx);
	AS_API int                    asEngine_SetContextCallbacks(asIScriptEngine *e, asREQUESTCONTEXTFUNC_t requestCtx, asRETURNCONTEXTFUNC_t returnCtx, void *param);

	// String interpretation
	AS_API asETokenClass      asEngine_ParseToken(asIScriptEngine *e, const char *string, size_t stringLength, asUINT *tokenLength);

	// Garbage collection
	AS_API int                asEngine_GarbageCollect(asIScriptEngine *e, asDWORD flags);
	AS_API void               asEngine_GetGCStatistics(asIScriptEngine *e, asUINT *currentSize, asUINT *totalDestroyed, asUINT *totalDetected, asUINT *newObjects, asUINT *totalNewDestroyed);
	AS_API int                asEngine_NotifyGarbageCollectorOfNewObject(asIScriptEngine *e, void *obj, asITypeInfo *type);
	AS_API int                asEngine_GetObjectInGC(asIScriptEngine *e, asUINT idx, asUINT *seqNbr, void **obj, asITypeInfo **type);
	AS_API void               asEngine_GCEnumCallback(asIScriptEngine *e, void *obj);

	// User data
	AS_API void *             asEngine_SetUserData(asIScriptEngine *e, void *data, asPWORD type);
	AS_API void *             asEngine_GetUserData(asIScriptEngine *e, asPWORD type);
	AS_API void               asEngine_SetEngineUserDataCleanupCallback(asIScriptEngine *e, asCLEANENGINEFUNC_t callback, asPWORD type);
	AS_API void               asEngine_SetModuleUserDataCleanupCallback(asIScriptEngine *e, asCLEANMODULEFUNC_t callback);
	AS_API void               asEngine_SetContextUserDataCleanupCallback(asIScriptEngine *e, asCLEANCONTEXTFUNC_t callback);
	AS_API void               asEngine_SetFunctionUserDataCleanupCallback(asIScriptEngine *e, asCLEANFUNCTIONFUNC_t callback);
	AS_API void               asEngine_SetTypeInfoUserDataCleanupCallback(asIScriptEngine *e, asCLEANTYPEINFOFUNC_t callback, asPWORD type);
	AS_API void               asEngine_SetScriptObjectUserDataCleanupCallback(asIScriptEngine *e, asCLEANSCRIPTOBJECTFUNC_t callback, asPWORD type);

	///////////////////////////////////////////
	// asIScriptModule

	AS_API asIScriptEngine   *asModule_GetEngine(asIScriptModule *m);
	AS_API void               asModule_SetName(asIScriptModule *m, const char *name);
	AS_API const char        *asModule_GetName(asIScriptModule *m); 
	AS_API void               asModule_Discard(asIScriptModule *m);

	// Compilation
	AS_API int                asModule_AddScriptSection(asIScriptModule *m, const char *name, const char *code, size_t codeLength, int lineOffset);
	AS_API int                asModule_Build(asIScriptModule *m);
	AS_API int                asModule_CompileFunction(asIScriptModule *m, const char *sectionName, const char *code, int lineOffset, asDWORD compileFlags, asIScriptFunction **outFunc);
	AS_API int                asModule_CompileGlobalVar(asIScriptModule *m, const char *sectionName, const char *code, int lineOffset);
	AS_API asDWORD            asModule_SetAccessMask(asIScriptModule *m, asDWORD accessMask);
	AS_API int                asModule_SetDefaultNamespace(asIScriptModule *m,const char *nameSpace);
	AS_API const char        *asModule_GetDefaultNamespace(asIScriptModule *m);

	// Functions
	AS_API asUINT             asModule_GetFunctionCount(asIScriptModule *m);
	AS_API asIScriptFunction *asModule_GetFunctionByIndex(asIScriptModule *m, asUINT index);
	AS_API asIScriptFunction *asModule_GetFunctionByDecl(asIScriptModule *m, const char *decl);
	AS_API asIScriptFunction *asModule_GetFunctionByName(asIScriptModule *m, const char *name); 
	AS_API int                asModule_RemoveFunction(asIScriptModule *m, asIScriptFunction *func);

	// Global variables
	AS_API int                asModule_ResetGlobalVars(asIScriptModule *m, asIScriptContext *ctx);
	AS_API asUINT             asModule_GetGlobalVarCount(asIScriptModule *m);
	AS_API int                asModule_GetGlobalVarIndexByName(asIScriptModule *m, const char *name);
	AS_API int                asModule_GetGlobalVarIndexByDecl(asIScriptModule *m, const char *decl);
	AS_API const char        *asModule_GetGlobalVarDeclaration(asIScriptModule *m, asUINT index, asBOOL includeNamespace);
	AS_API int                asModule_GetGlobalVar(asIScriptModule *m, asUINT index, const char **name, const char **nameSpace, int *typeId, asBOOL *isConst);
	AS_API void              *asModule_GetAddressOfGlobalVar(asIScriptModule *m, asUINT index);
	AS_API int                asModule_RemoveGlobalVar(asIScriptModule *m, asUINT index);

	// Type identification
	AS_API asUINT             asModule_GetObjectTypeCount(asIScriptModule *m);
	AS_API asITypeInfo       *asModule_GetObjectTypeByIndex(asIScriptModule *m, asUINT index);
	AS_API int                asModule_GetTypeIdByDecl(asIScriptModule *m, const char *decl);
	AS_API asITypeInfo       *asModule_GetTypeInfoByName(asIScriptModule *m, const char *name);
	AS_API asITypeInfo       *asModule_GetTypeInfoByDecl(asIScriptModule *m, const char *decl);

	// Enums
	AS_API asUINT             asModule_GetEnumCount(asIScriptModule *m);
	AS_API asITypeInfo *      asModule_GetEnumByIndex(asIScriptModule *m, asUINT index);

	// Typedefs
	AS_API asUINT             asModule_GetTypedefCount(asIScriptModule *m);
	AS_API asITypeInfo *      asModule_GetTypedefByIndex(asIScriptModule *m, asUINT index);

	// Dynamic binding between modules
	AS_API asUINT             asModule_GetImportedFunctionCount(asIScriptModule *m);
	AS_API int                asModule_GetImportedFunctionIndexByDecl(asIScriptModule *m, const char *decl);
	AS_API const char        *asModule_GetImportedFunctionDeclaration(asIScriptModule *m, asUINT importIndex);
	AS_API const char        *asModule_GetImportedFunctionSourceModule(asIScriptModule *m, asUINT importIndex);
	AS_API int                asModule_BindImportedFunction(asIScriptModule *m, asUINT importIndex, asIScriptFunction *func);
	AS_API int                asModule_UnbindImportedFunction(asIScriptModule *m, asUINT importIndex);
	AS_API int                asModule_BindAllImportedFunctions(asIScriptModule *m);
	AS_API int                asModule_UnbindAllImportedFunctions(asIScriptModule *m);

	// Bytecode saving and loading
	AS_API int                asModule_SaveByteCode(asIScriptModule *m, asIBinaryStream *out, asBOOL stripDebugInfo);
	AS_API int                asModule_LoadByteCode(asIScriptModule *m, asIBinaryStream *in, asBOOL *wasDebugInfoStripped);

	// User data
	AS_API void              *asModule_SetUserData(asIScriptModule *m, void *data);
	AS_API void              *asModule_GetUserData(asIScriptModule *m);

	///////////////////////////////////////////
	// asIScriptContext

	// Memory management
	AS_API int              asContext_AddRef(asIScriptContext *c);
	AS_API int              asContext_Release(asIScriptContext *c);

	// Miscellaneous
	AS_API asIScriptEngine *asContext_GetEngine(asIScriptContext *c);

	// Execution
	AS_API int              asContext_Prepare(asIScriptContext *c, asIScriptFunction *func);
	AS_API int              asContext_Unprepare(asIScriptContext *c);
	AS_API int              asContext_Execute(asIScriptContext *c);
	AS_API int              asContext_Abort(asIScriptContext *c);
	AS_API int              asContext_Suspend(asIScriptContext *c);
	AS_API asEContextState  asContext_GetState(asIScriptContext *c);
	AS_API int              asContext_PushState(asIScriptContext *c);
	AS_API int              asContext_PopState(asIScriptContext *c);
	AS_API asBOOL           asContext_IsNested(asIScriptContext *c, asUINT *nestCount);

	// Object pointer for calling class methods
	AS_API int              asContext_SetObject(asIScriptContext *c, void *obj);

	// Arguments
	AS_API int              asContext_SetArgByte(asIScriptContext *c, asUINT arg, asBYTE value);
	AS_API int              asContext_SetArgWord(asIScriptContext *c, asUINT arg, asWORD value);
	AS_API int              asContext_SetArgDWord(asIScriptContext *c, asUINT arg, asDWORD value);
	AS_API int              asContext_SetArgQWord(asIScriptContext *c, asUINT arg, asQWORD value);
	AS_API int              asContext_SetArgFloat(asIScriptContext *c, asUINT arg, float value);
	AS_API int              asContext_SetArgDouble(asIScriptContext *c, asUINT arg, double value);
	AS_API int              asContext_SetArgAddress(asIScriptContext *c, asUINT arg, void *addr);
	AS_API int              asContext_SetArgObject(asIScriptContext *c, asUINT arg, void *obj);
	AS_API int              asContext_SetArgVarType(asIScriptContext *c, asUINT arg, void *ptr, int typeId);
	AS_API void *           asContext_GetAddressOfArg(asIScriptContext *c, asUINT arg);

	// Return value
	AS_API asBYTE           asContext_GetReturnByte(asIScriptContext *c);
	AS_API asWORD           asContext_GetReturnWord(asIScriptContext *c);
	AS_API asDWORD          asContext_GetReturnDWord(asIScriptContext *c);
	AS_API asQWORD          asContext_GetReturnQWord(asIScriptContext *c);
	AS_API float            asContext_GetReturnFloat(asIScriptContext *c);
	AS_API double           asContext_GetReturnDouble(asIScriptContext *c);
	AS_API void *           asContext_GetReturnAddress(asIScriptContext *c);
	AS_API void *           asContext_GetReturnObject(asIScriptContext *c);
	AS_API void *           asContext_GetAddressOfReturnValue(asIScriptContext *c);

	// Exception handling
	AS_API int                asContext_SetException(asIScriptContext *c, const char *string);
	AS_API int                asContext_GetExceptionLineNumber(asIScriptContext *c, int *column, const char **sectionName);
	AS_API asIScriptFunction *asContext_GetExceptionFunction(asIScriptContext *c);
	AS_API const char *       asContext_GetExceptionString(asIScriptContext *c);
	AS_API int                asContext_SetExceptionCallback(asIScriptContext *c, asFUNCTION_t callback, void *obj, int callConv);
	AS_API void               asContext_ClearExceptionCallback(asIScriptContext *c);

	// Debugging
	AS_API int                asContext_SetLineCallback(asIScriptContext *c, asFUNCTION_t callback, void *obj, int callConv);
	AS_API void               asContext_ClearLineCallback(asIScriptContext *c);
	AS_API asUINT             asContext_GetCallstackSize(asIScriptContext *c);
	AS_API asIScriptFunction *asContext_GetFunction(asIScriptContext *c, asUINT stackLevel);
	AS_API int                asContext_GetLineNumber(asIScriptContext *c, asUINT stackLevel, int *column, const char **sectionName);
	AS_API int                asContext_GetVarCount(asIScriptContext *c, asUINT stackLevel);
	AS_API const char *       asContext_GetVarName(asIScriptContext *c, asUINT varIndex, asUINT stackLevel);
	AS_API const char *       asContext_GetVarDeclaration(asIScriptContext *c, asUINT varIndex, asUINT stackLevel, asBOOL includeNamespace);
	AS_API int                asContext_GetVarTypeId(asIScriptContext *c, asUINT varIndex, asUINT stackLevel);
	AS_API void *             asContext_GetAddressOfVar(asIScriptContext *c, asUINT varIndex, asUINT stackLevel);
	AS_API asBOOL             asContext_IsVarInScope(asIScriptContext *c, asUINT varIndex, asUINT stackLevel);
	AS_API int                asContext_GetThisTypeId(asIScriptContext *c, asUINT stackLevel);
	AS_API void *             asContext_GetThisPointer(asIScriptContext *c, asUINT stackLevel);
	AS_API asIScriptFunction *asContext_GetSystemFunction(asIScriptContext *c);

	// User data
	AS_API void *           asContext_SetUserData(asIScriptContext *c, void *data);
	AS_API void *           asContext_GetUserData(asIScriptContext *c);

	///////////////////////////////////////////
	// asIScriptGeneric

	// Miscellaneous
	AS_API asIScriptEngine   *asGeneric_GetEngine(asIScriptGeneric *g);
	AS_API asIScriptFunction *asGeneric_GetFunction(asIScriptGeneric *g);
	AS_API void              *asGeneric_GetAuxiliary(asIScriptGeneric *g);

	// Object
	AS_API void *           asGeneric_GetObject(asIScriptGeneric *g);
	AS_API int              asGeneric_GetObjectTypeId(asIScriptGeneric *g);

	// Arguments
	AS_API int              asGeneric_GetArgCount(asIScriptGeneric *g);
	AS_API int              asGeneric_GetArgTypeId(asIScriptGeneric *g, asUINT arg, asDWORD *flags);
	AS_API asBYTE           asGeneric_GetArgByte(asIScriptGeneric *g, asUINT arg);
	AS_API asWORD           asGeneric_GetArgWord(asIScriptGeneric *g, asUINT arg);
	AS_API asDWORD          asGeneric_GetArgDWord(asIScriptGeneric *g, asUINT arg);
	AS_API asQWORD          asGeneric_GetArgQWord(asIScriptGeneric *g, asUINT arg);
	AS_API float            asGeneric_GetArgFloat(asIScriptGeneric *g, asUINT arg);
	AS_API double           asGeneric_GetArgDouble(asIScriptGeneric *g, asUINT arg);
	AS_API void *           asGeneric_GetArgAddress(asIScriptGeneric *g, asUINT arg);
	AS_API void *           asGeneric_GetArgObject(asIScriptGeneric *g, asUINT arg);
	AS_API void *           asGeneric_GetAddressOfArg(asIScriptGeneric *g, asUINT arg);

	// Return value
	AS_API int              asGeneric_GetReturnTypeId(asIScriptGeneric *g, asDWORD *flags);
	AS_API int              asGeneric_SetReturnByte(asIScriptGeneric *g, asBYTE val);
	AS_API int              asGeneric_SetReturnWord(asIScriptGeneric *g, asWORD val);
	AS_API int              asGeneric_SetReturnDWord(asIScriptGeneric *g, asDWORD val);
	AS_API int              asGeneric_SetReturnQWord(asIScriptGeneric *g, asQWORD val);
	AS_API int              asGeneric_SetReturnFloat(asIScriptGeneric *g, float val);
	AS_API int              asGeneric_SetReturnDouble(asIScriptGeneric *g, double val);
	AS_API int              asGeneric_SetReturnAddress(asIScriptGeneric *g, void *addr);
	AS_API int              asGeneric_SetReturnObject(asIScriptGeneric *g, void *obj);
	AS_API void *           asGeneric_GetAddressOfReturnLocation(asIScriptGeneric *g);

	///////////////////////////////////////////
	// asIScriptObject

	// Memory management
	AS_API int                    asObject_AddRef(asIScriptObject *s);
	AS_API int                    asObject_Release(asIScriptObject *s);
	AS_API asILockableSharedBool *asObject_GetWeakRefFlag(asIScriptObject *s);

	// Type info
	AS_API int            asObject_GetTypeId(asIScriptObject *s);
	AS_API asITypeInfo *  asObject_GetObjectType(asIScriptObject *s);

	// Class properties
	AS_API asUINT           asObject_GetPropertyCount(asIScriptObject *s);
	AS_API int              asObject_GetPropertyTypeId(asIScriptObject *s, asUINT prop);
	AS_API const char *     asObject_GetPropertyName(asIScriptObject *s, asUINT prop);
	AS_API void *           asObject_GetAddressOfProperty(asIScriptObject *s, asUINT prop);

	// Miscellaneous
	AS_API asIScriptEngine *asObject_GetEngine(asIScriptObject *s);
	AS_API int              asObject_CopyFrom(asIScriptObject *s, asIScriptObject *other);

	// User data
	AS_API void *asObject_SetUserData(asIScriptObject *s, void *data, asPWORD type);
	AS_API void *asObject_GetUserData(asIScriptObject *s, asPWORD type);

	///////////////////////////////////////////
	// asITypeInfo

	// Miscellaneous
	AS_API asIScriptEngine *asTypeInfo_GetEngine(asITypeInfo *o);
	AS_API const char      *asTypeInfo_GetConfigGroup(asITypeInfo *o);
	AS_API asDWORD          asTypeInfo_GetAccessMask(asITypeInfo *o);
	AS_API asIScriptModule *asTypeInfo_GetModule(asITypeInfo *o);

	// Memory management
	AS_API int              asTypeInfo_AddRef(const asITypeInfo *o);
	AS_API int              asTypeInfo_Release(const asITypeInfo *o);

	// Type info
	AS_API const char      *asTypeInfo_GetName(const asITypeInfo *o);
	AS_API const char      *asTypeInfo_GetNamespace(const asITypeInfo *o);
	AS_API asITypeInfo     *asTypeInfo_GetBaseType(const asITypeInfo *o);
	AS_API asBOOL           asTypeInfo_DerivesFrom(const asITypeInfo *o, const asITypeInfo *objType);
	AS_API asDWORD          asTypeInfo_GetFlags(const asITypeInfo *o);
	AS_API asUINT           asTypeInfo_GetSize(const asITypeInfo *o);
	AS_API int              asTypeInfo_GetTypeId(const asITypeInfo *o);
	AS_API int              asTypeInfo_GetSubTypeId(const asITypeInfo *o, asUINT subTypeIndex);
	AS_API asITypeInfo     *asTypeInfo_GetSubType(const asITypeInfo *o, asUINT subTypeIndex);
	AS_API asUINT           asTypeInfo_GetSubTypeCount(const asITypeInfo *o);

	// Interfaces
	AS_API asUINT           asTypeInfo_GetInterfaceCount(const asITypeInfo *o);
	AS_API asITypeInfo     *asTypeInfo_GetInterface(const asITypeInfo *o, asUINT index);
	AS_API asBOOL           asTypeInfo_Implements(const asITypeInfo *o, const asITypeInfo *objType); 

	// Factories
	AS_API asUINT             asTypeInfo_GetFactoryCount(const asITypeInfo *o);
	AS_API asIScriptFunction *asTypeInfo_GetFactoryByIndex(const asITypeInfo *o, asUINT index);
	AS_API asIScriptFunction *asTypeInfo_GetFactoryByDecl(const asITypeInfo *o, const char *decl);

	// Methods
	AS_API asUINT             asTypeInfo_GetMethodCount(const asITypeInfo *o);
	AS_API asIScriptFunction *asTypeInfo_GetMethodByIndex(const asITypeInfo *o, asUINT index, asBOOL getVirtual);
	AS_API asIScriptFunction *asTypeInfo_GetMethodByName(const asITypeInfo *o, const char *name, asBOOL getVirtual);
	AS_API asIScriptFunction *asTypeInfo_GetMethodByDecl(const asITypeInfo *o, const char *decl, asBOOL getVirtual);

	// Properties
	AS_API asUINT      asTypeInfo_GetPropertyCount(const asITypeInfo *o);
	AS_API int         asTypeInfo_GetProperty(const asITypeInfo *o, asUINT index, const char **name, int *typeId, asBOOL *isPrivate, asBOOL *isProtected, int *offset, asBOOL *isReference, asDWORD *accessMask);
	AS_API const char *asTypeInfo_GetPropertyDeclaration(const asITypeInfo *o, asUINT index);

	// Behaviours
	AS_API asUINT             asTypeInfo_GetBehaviourCount(const asITypeInfo *o);
	AS_API asIScriptFunction *asTypeInfo_GetBehaviourByIndex(const asITypeInfo *o, asUINT index, asEBehaviours *outBehaviour);

	// Child types
	AS_API asUINT       asTypeInfo_GetChildFuncdefCount(asITypeInfo *o);
	AS_API asITypeInfo *asTypeInfo_GetChildFuncdef(asITypeInfo *o, asUINT index);
	AS_API asITypeInfo *asTypeInfo_GetParentType(asITypeInfo *o);

	// Enums
	AS_API asUINT      asTypeInfo_GetEnumValueCount(asITypeInfo *o);
	AS_API const char *asTypeInfo_GetEnumValueByIndex(asITypeInfo *o, asUINT index, int *outValue);

	// Typedef
	AS_API int asTypeInfo_GetTypedefTypeId(asITypeInfo *o);

	// Funcdef
	AS_API asIScriptFunction *asTypeInfo_GetFuncdefSignature(asITypeInfo *o);

	// User data
	AS_API void            *asTypeInfo_SetUserData(asITypeInfo *o, void *data, asPWORD type);
	AS_API void            *asTypeInfo_GetUserData(asITypeInfo *o, asPWORD type);


	///////////////////////////////////////////
	// asIScriptFunction

	AS_API asIScriptEngine *asFunction_GetEngine(const asIScriptFunction *f);

	// Memory management
	AS_API int              asFunction_AddRef(const asIScriptFunction *f);
	AS_API int              asFunction_Release(const asIScriptFunction *f);

	// Miscellaneous
	AS_API int              asFunction_GetId(const asIScriptFunction *f);
	AS_API asEFuncType      asFunction_GetFuncType(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetModuleName(const asIScriptFunction *f);
	AS_API asIScriptModule *asFunction_GetModule(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetScriptSectionName(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetConfigGroup(const asIScriptFunction *f);
	AS_API asDWORD          asFunction_GetAccessMask(const asIScriptFunction *f);
	AS_API void            *asFunction_GetAuxiliary(const asIScriptFunction *f);

	// Function signature
	AS_API asITypeInfo     *asFunction_GetObjectType(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetObjectName(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetName(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetNamespace(const asIScriptFunction *f);
	AS_API const char      *asFunction_GetDeclaration(const asIScriptFunction *f, asBOOL includeObjectName, asBOOL includeNamespace);
	AS_API asBOOL           asFunction_IsReadOnly(const asIScriptFunction *f);
	AS_API asBOOL           asFunction_IsPrivate(const asIScriptFunction *f);
	AS_API asBOOL           asFunction_IsProtected(const asIScriptFunction *f);
	AS_API asBOOL           asFunction_IsFinal(const asIScriptFunction *f);
	AS_API asBOOL           asFunction_IsOverride(const asIScriptFunction *f);
	AS_API asBOOL           asFunction_IsShared(const asIScriptFunction *f);
	AS_API asUINT           asFunction_GetParamCount(const asIScriptFunction *f);
	AS_API int              asFunction_GetParam(const asIScriptFunction *f, asUINT index, int *typeId, asDWORD *flags, const char **name, const char **defaultArg);
	AS_API int              asFunction_GetReturnTypeId(const asIScriptFunction *f);

	// Type id for function pointers
	AS_API int              asFunction_GetTypeId(const asIScriptFunction *f);
	AS_API asBOOL           asFunction_IsCompatibleWithTypeId(const asIScriptFunction *f, int typeId);

	// Delegates
	AS_API void              *asFunction_GetDelegateObject(const asIScriptFunction *f);
	AS_API asITypeInfo       *asFunction_GetDelegateObjectType(const asIScriptFunction *f);
	AS_API asIScriptFunction *asFunction_GetDelegateFunction(const asIScriptFunction *f);

	// Debug information
	AS_API asUINT           asFunction_GetVarCount(const asIScriptFunction *f);
	AS_API int              asFunction_GetVar(const asIScriptFunction *f, asUINT index, const char **name, int *typeId);
	AS_API const char *     asFunction_GetVarDecl(const asIScriptFunction *f, asUINT index, asBOOL includeNamespace);
	AS_API int              asFunction_FindNextLineWithCode(const asIScriptFunction *f, int line);

	// For JIT compilation
	AS_API asDWORD         *asFunction_GetByteCode(asIScriptFunction *f, asUINT *length);

	// User data
	AS_API void            *asFunction_SetUserData(asIScriptFunction *f, void *userData);
	AS_API void            *asFunction_GetUserData(const asIScriptFunction *f);

#ifdef __cplusplus
}
#endif
#endif // ANGELSCRIPT_DLL_MANUAL_IMPORT

END_AS_NAMESPACE

#endif
