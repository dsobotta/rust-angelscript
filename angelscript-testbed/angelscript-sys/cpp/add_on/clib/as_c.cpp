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
// as_c.cpp
//
// A C interface to the library
//

// Include the C++ interface header so we can call the proper methods
#include <angelscript.h>

BEGIN_AS_NAMESPACE

typedef void (*asBINARYREADFUNC_t)(void *ptr, asUINT size, void *param);
typedef void (*asBINARYWRITEFUNC_t)(const void *ptr, asUINT size, void *param);

typedef enum { asTRUE = 1, asFALSE = 0 } asBOOL;

class asCBinaryStreamC : public asIBinaryStream
{
public:
	asCBinaryStreamC(asBINARYWRITEFUNC_t write, asBINARYREADFUNC_t read, void *param) {this->write = write; this->read = read; this->param = param;}

	void Write(const void *ptr, asUINT size) { write(ptr, size, param); }
	void Read(void *ptr, asUINT size) { read(ptr, size, param); }

	asBINARYREADFUNC_t read;
	asBINARYWRITEFUNC_t write;
	void *param;
};

extern "C"
{
	// Implement a global wrapper function for each of the library's interface methods
	// Note that half the implementations are waaaay off to the right ---->

	///////////////////////////////////////////
	// asIScriptEngine

	// Memory management
	AS_API int                asEngine_AddRef(asIScriptEngine *e)                                                                                                                                      { return e->AddRef(); }
	AS_API int                asEngine_Release(asIScriptEngine *e)                                                                                                                                     { return e->Release(); }
	AS_API int                asEngine_ShutDownAndRelease(asIScriptEngine *e)                                                                                                                          { return e->ShutDownAndRelease(); }

	// Engine properties
	AS_API int                asEngine_SetEngineProperty(asIScriptEngine *e, asEEngineProp property, asPWORD value)                                                                                    { return e->SetEngineProperty(property, value); }
	AS_API asPWORD            asEngine_GetEngineProperty(asIScriptEngine *e, asEEngineProp property)                                                                                                   { return e->GetEngineProperty(property); }

	// Compiler messages
	AS_API int                asEngine_SetMessageCallback(asIScriptEngine *e, asFUNCTION_t callback, void *obj, asDWORD callConv)                                                                      { return e->SetMessageCallback(asFUNCTION(callback), obj, callConv); }
	AS_API int                asEngine_ClearMessageCallback(asIScriptEngine *e)                                                                                                                        { return e->ClearMessageCallback(); }
	AS_API int                asEngine_WriteMessage(asIScriptEngine *e, const char *section, int row, int col, asEMsgType type, const char *message)                                                   { return e->WriteMessage(section, row, col, type, message); }

	// JIT Compiler
	AS_API int                asEngine_SetJITCompiler(asIScriptEngine *e, asIJITCompiler *compiler)                                                                                                    { return e->SetJITCompiler(compiler); }
	AS_API asIJITCompiler *   asEngine_GetJITCompiler(asIScriptEngine *e)                                                                                                                              { return e->GetJITCompiler(); }

	// Global functions
	AS_API int                asEngine_RegisterGlobalFunction(asIScriptEngine *e, const char *declaration, asFUNCTION_t funcPointer, asDWORD callConv, void *auxiliary)                                                 { return e->RegisterGlobalFunction(declaration, asFUNCTION(funcPointer), callConv, auxiliary); }
	AS_API asUINT             asEngine_GetGlobalFunctionCount(asIScriptEngine *e)                                                                                                                      { return e->GetGlobalFunctionCount(); }
	AS_API asIScriptFunction* asEngine_GetGlobalFunctionByIndex(asIScriptEngine *e, asUINT index)                                                                                                      { return e->GetGlobalFunctionByIndex(index); }
	AS_API asIScriptFunction* asEngine_GetGlobalFunctionByDecl(asIScriptEngine *e, const char *declaration)                                                                                            { return e->GetGlobalFunctionByDecl(declaration); }

	// Global properties
	AS_API int                asEngine_RegisterGlobalProperty(asIScriptEngine *e, const char *declaration, void *pointer)                                                                              { return e->RegisterGlobalProperty(declaration, pointer); }
	AS_API asUINT             asEngine_GetGlobalPropertyCount(asIScriptEngine *e)                                                                                                                      { return e->GetGlobalPropertyCount(); }
	AS_API int                asEngine_GetGlobalPropertyByIndex(asIScriptEngine *e, asUINT index, const char **name, const char **nameSpace = 0, int *typeId = 0, asBOOL *isConst = 0, const char **configGroup = 0, void **pointer = 0, asDWORD *accessMask = 0)     { bool _isConst; int r = e->GetGlobalPropertyByIndex(index, name, nameSpace, typeId, &_isConst, configGroup, pointer, accessMask); if( isConst ) *isConst = _isConst ? asTRUE : asFALSE; return r; }
	AS_API int                asEngine_GetGlobalPropertyIndexByName(asIScriptEngine *e, const char *name)                                                                                              { return e->GetGlobalPropertyIndexByName(name); }
	AS_API int                asEngine_GetGlobalPropertyIndexByDecl(asIScriptEngine *e, const char *decl)                                                                                              { return e->GetGlobalPropertyIndexByDecl(decl); }

	// Object types
	AS_API int                asEngine_RegisterObjectType(asIScriptEngine *e, const char *name, int byteSize, asDWORD flags)                                                                           { return e->RegisterObjectType(name, byteSize, flags); }
	AS_API int                asEngine_RegisterObjectProperty(asIScriptEngine *e, const char *obj, const char *declaration, int byteOffset)                                                            { return e->RegisterObjectProperty(obj, declaration, byteOffset); }
	AS_API int                asEngine_RegisterObjectMethod(asIScriptEngine *e, const char *obj, const char *declaration, asFUNCTION_t funcPointer, asDWORD callConv, void *auxiliary)                                  { return e->RegisterObjectMethod(obj, declaration, asFUNCTION(funcPointer), callConv, auxiliary); }
	AS_API int                asEngine_RegisterObjectBehaviour(asIScriptEngine *e, const char *datatype, asEBehaviours behaviour, const char *declaration, asFUNCTION_t funcPointer, asDWORD callConv, void *auxiliary) { return e->RegisterObjectBehaviour(datatype, behaviour, declaration, asFUNCTION(funcPointer), callConv, auxiliary); }
	AS_API int                asEngine_RegisterInterface(asIScriptEngine *e, const char *name)                                                                                                         { return e->RegisterInterface(name); }
	AS_API int                asEngine_RegisterInterfaceMethod(asIScriptEngine *e, const char *intf, const char *declaration)                                                                          { return e->RegisterInterfaceMethod(intf, declaration); }
	AS_API asUINT             asEngine_GetObjectTypeCount(asIScriptEngine *e)                                                                                                                          { return e->GetObjectTypeCount(); }
	AS_API asITypeInfo *      asEngine_GetObjectTypeByIndex(asIScriptEngine *e, asUINT index)                                                                                                          { return e->GetObjectTypeByIndex(index); }

	// String factory
	AS_API int                asEngine_RegisterStringFactory(asIScriptEngine *e, const char *datatype, asFUNCTION_t factoryFunc, asDWORD callConv, void *auxiliary)                                    { return e->RegisterStringFactory(datatype, asFUNCTION(factoryFunc), callConv, auxiliary); }
	AS_API int                asEngine_GetStringFactoryReturnTypeId(asIScriptEngine *e, asDWORD *flags)                                                                                                                { return e->GetStringFactoryReturnTypeId(flags); }

	// Default array type
	AS_API int                asEngine_RegisterDefaultArrayType(asIScriptEngine *e, const char *type)                                                                                                  { return e->RegisterDefaultArrayType(type); }
	AS_API int                asEngine_GetDefaultArrayTypeId(asIScriptEngine *e)                                                                                                                       { return e->GetDefaultArrayTypeId(); }

	// Enums
	AS_API int                asEngine_RegisterEnum(asIScriptEngine *e, const char *type)                                                                                                              { return e->RegisterEnum(type); }
	AS_API int                asEngine_RegisterEnumValue(asIScriptEngine *e, const char *type, const char *name, int value)                                                                            { return e->RegisterEnumValue(type,name,value); }
	AS_API asUINT             asEngine_GetEnumCount(asIScriptEngine *e)                                                                                                                                { return e->GetEnumCount(); }
	AS_API asITypeInfo *      asEngine_GetEnumByIndex(asIScriptEngine *e, asUINT index)                                                                                                                { return e->GetEnumByIndex(index); }

	// Funcdefs
	AS_API int                asEngine_RegisterFuncdef(asIScriptEngine *e, const char *decl)                                                                                                           { return e->RegisterFuncdef(decl); }
	AS_API asUINT             asEngine_GetFuncdefCount(asIScriptEngine *e)                                                                                                                             { return e->GetFuncdefCount(); }
	AS_API asITypeInfo *      asEngine_GetFuncdefByIndex(asIScriptEngine *e, asUINT index)                                                                                                             { return e->GetFuncdefByIndex(index); }

	// Typedefs
	AS_API int                asEngine_RegisterTypedef(asIScriptEngine *e, const char *type, const char *decl)                                                                                         { return e->RegisterTypedef(type,decl); }
	AS_API asUINT             asEngine_GetTypedefCount(asIScriptEngine *e)                                                                                                                             { return e->GetTypedefCount(); }
	AS_API asITypeInfo *      asEngine_GetTypedefByIndex(asIScriptEngine *e, asUINT index)                                                                                                             { return e->GetTypedefByIndex(index); }

	// Configuration groups
	AS_API int                asEngine_BeginConfigGroup(asIScriptEngine *e, const char *groupName)                                                                                                     { return e->BeginConfigGroup(groupName); }
	AS_API int                asEngine_EndConfigGroup(asIScriptEngine *e)                                                                                                                              { return e->EndConfigGroup(); }
	AS_API int                asEngine_RemoveConfigGroup(asIScriptEngine *e, const char *groupName)                                                                                                    { return e->RemoveConfigGroup(groupName); }
	AS_API asDWORD            asEngine_SetDefaultAccessMask(asIScriptEngine *e, asDWORD defaultMask)                                                                                                 { return e->SetDefaultAccessMask(defaultMask); }
	AS_API int                asEngine_SetDefaultNamespace(asIScriptEngine *e, const char *nameSpace)                                                                                                { return e->SetDefaultNamespace(nameSpace); }
	AS_API const char *       asEngine_GetDefaultNamespace(asIScriptEngine *e)                                                                                                                         { return e->GetDefaultNamespace(); }

	// Script modules
	AS_API asIScriptModule *  asEngine_GetModule(asIScriptEngine *e, const char *module, asEGMFlags flag)                                                                                              { return e->GetModule(module, flag); }
	AS_API int                asEngine_DiscardModule(asIScriptEngine *e, const char *module)                                                                                                           { return e->DiscardModule(module); }
	AS_API asUINT             asEngine_GetModuleCount(asIScriptEngine *e)                                                                                                                              { return e->GetModuleCount(); }
	AS_API asIScriptModule *  asEngine_GetModuleByIndex(asIScriptEngine *e, asUINT index)                                                                                                              { return e->GetModuleByIndex(index); }

	// Script functions
	AS_API asIScriptFunction *asEngine_GetFunctionById(asIScriptEngine *e, int funcId)                                                                                                                { return e->GetFunctionById(funcId); }

	// Type identification
	AS_API int                asEngine_GetTypeIdByDecl(asIScriptEngine *e, const char *decl)                                                                                                           { return e->GetTypeIdByDecl(decl); }
	AS_API const char *       asEngine_GetTypeDeclaration(asIScriptEngine *e, int typeId, asBOOL includeNamespace)                                                                                     { return e->GetTypeDeclaration(typeId, includeNamespace ? true : false); }
	AS_API int                asEngine_GetSizeOfPrimitiveType(asIScriptEngine *e, int typeId)                                                                                                          { return e->GetSizeOfPrimitiveType(typeId); }
	AS_API asITypeInfo   *asEngine_GetTypeInfoById(asIScriptEngine *e, int typeId)         { return e->GetTypeInfoById(typeId); }
	AS_API asITypeInfo   *asEngine_GetTypeInfoByName(asIScriptEngine *e, const char *name) { return e->GetTypeInfoByName(name); }
	AS_API asITypeInfo   *asEngine_GetTypeInfoByDecl(asIScriptEngine *e, const char *decl) { return e->GetTypeInfoByDecl(decl); }

	// Script execution
	AS_API asIScriptContext *     asEngine_CreateContext(asIScriptEngine *e)                                                                                                                               { return e->CreateContext(); }
	AS_API void *                 asEngine_CreateScriptObject(asIScriptEngine *e, asITypeInfo *type)                                                                                                       { return e->CreateScriptObject(type); }
	AS_API void *                 asEngine_CreateScriptObjectCopy(asIScriptEngine *e, void *obj, asITypeInfo *type)                                                                                        { return e->CreateScriptObjectCopy(obj, type); }
	AS_API void *                 asEngine_CreateUninitializedScriptObject(asIScriptEngine *e, asITypeInfo *type)                                                                                          { return e->CreateUninitializedScriptObject(type); }
	AS_API asIScriptFunction *    asEngine_CreateDelegate(asIScriptEngine *e, asIScriptFunction *func, void *obj)                                                                                          { return e->CreateDelegate(func, obj); }
	AS_API int                    asEngine_AssignScriptObject(asIScriptEngine *e, void *dstObj, void *srcObj, asITypeInfo *type)                                                                           { return e->AssignScriptObject(dstObj, srcObj, type); }
	AS_API void                   asEngine_ReleaseScriptObject(asIScriptEngine *e, void *obj, asITypeInfo *type)                                                                                           { e->ReleaseScriptObject(obj, type); }
	AS_API void                   asEngine_AddRefScriptObject(asIScriptEngine *e, void *obj, asITypeInfo *type)                                                                                            { e->AddRefScriptObject(obj, type); }
	AS_API int                    asEngine_RefCastObject(asIScriptEngine *e, void *obj, asITypeInfo *fromType, asITypeInfo *toType, void **newPtr, bool useOnlyImplicitCast)                               { return e->RefCastObject(obj, fromType, toType, newPtr, useOnlyImplicitCast); }
	AS_API asILockableSharedBool *asEngine_GetWeakRefFlagOfScriptObject(asIScriptEngine *e, void *obj, asITypeInfo *type)                                                                                  { return e->GetWeakRefFlagOfScriptObject(obj, type); }

	// Context pooling
	AS_API asIScriptContext      *asEngine_RequestContext(asIScriptEngine *e) { return e->RequestContext(); }
	AS_API void                   asEngine_ReturnContext(asIScriptEngine *e, asIScriptContext *ctx) { e->ReturnContext(ctx); }
	AS_API int                    asEngine_SetContextCallbacks(asIScriptEngine *e, asREQUESTCONTEXTFUNC_t requestCtx, asRETURNCONTEXTFUNC_t returnCtx, void *param) { return e->SetContextCallbacks(requestCtx, returnCtx, param); }

	// String interpretation
	AS_API asETokenClass      asEngine_ParseToken(asIScriptEngine *e, const char *string, size_t stringLength, asUINT *tokenLength)                                                                       { return e->ParseToken(string, stringLength, tokenLength); }

	// Garbage collection
	AS_API int                asEngine_GarbageCollect(asIScriptEngine *e, asDWORD flags)                                                                                                               { return e->GarbageCollect(flags); }
	AS_API void               asEngine_GetGCStatistics(asIScriptEngine *e, asUINT *currentSize, asUINT *totalDestroyed, asUINT *totalDetected, asUINT *newObjects, asUINT *totalNewDestroyed)          { e->GetGCStatistics(currentSize, totalDestroyed, totalDetected, newObjects, totalNewDestroyed); }
	AS_API int                asEngine_NotifyGarbageCollectorOfNewObject(asIScriptEngine *e, void *obj, asITypeInfo *type)                                                                             { return e->NotifyGarbageCollectorOfNewObject(obj, type); }
	AS_API int                asEngine_GetObjectInGC(asIScriptEngine *e, asUINT idx, asUINT *seqNbr, void **obj, asITypeInfo **type)                                                                   { return e->GetObjectInGC(idx, seqNbr, obj, type); }
	AS_API void               asEngine_GCEnumCallback(asIScriptEngine *e, void *obj)                                                                                                                   { e->GCEnumCallback(obj); }

	// User data
	AS_API void *             asEngine_SetUserData(asIScriptEngine *e, void *data, asPWORD type)                                                                                                       { return e->SetUserData(data, type); }
	AS_API void *             asEngine_GetUserData(asIScriptEngine *e, asPWORD type)                                                                                                                   { return e->GetUserData(type); }
	AS_API void               asEngine_SetEngineUserDataCleanupCallback(asIScriptEngine *e, asCLEANENGINEFUNC_t callback, asPWORD type)                                                                { e->SetEngineUserDataCleanupCallback(callback, type); }
	AS_API void               asEngine_SetModuleUserDataCleanupCallback(asIScriptEngine *e, asCLEANMODULEFUNC_t callback)                                                                              { e->SetModuleUserDataCleanupCallback(callback); }
	AS_API void               asEngine_SetContextUserDataCleanupCallback(asIScriptEngine *e, asCLEANCONTEXTFUNC_t callback)                                                                            { e->SetContextUserDataCleanupCallback(callback); }
	AS_API void               asEngine_SetFunctionUserDataCleanupCallback(asIScriptEngine *e, asCLEANFUNCTIONFUNC_t callback)                                                                          { e->SetFunctionUserDataCleanupCallback(callback); }
	AS_API void               asEngine_SetTypeInfoUserDataCleanupCallback(asIScriptEngine *e, asCLEANTYPEINFOFUNC_t callback, asPWORD type)                                                            { e->SetTypeInfoUserDataCleanupCallback(callback); }
	AS_API void               asEngine_SetScriptObjectUserDataCleanupCallback(asIScriptEngine *e, asCLEANSCRIPTOBJECTFUNC_t callback, asPWORD type)                                                    { e->SetScriptObjectUserDataCleanupCallback(callback, type); }

	///////////////////////////////////////////
	// asIScriptModule

	AS_API asIScriptEngine   *asModule_GetEngine(asIScriptModule *m)                                                                                                                                   { return m->GetEngine(); }
	AS_API void               asModule_SetName(asIScriptModule *m, const char *name)                                                                                                                   { m->SetName(name); }
	AS_API const char        *asModule_GetName(asIScriptModule *m)                                                                                                                                     { return m->GetName(); }
	AS_API void               asModule_Discard(asIScriptModule *m) { m->Discard(); }

	// Compilation
	AS_API int                asModule_AddScriptSection(asIScriptModule *m, const char *name, const char *code, size_t codeLength, int lineOffset)                                                     { return m->AddScriptSection(name, code, codeLength, lineOffset); }
	AS_API int                asModule_Build(asIScriptModule *m)                                                                                                                                       { return m->Build(); }
	AS_API int                asModule_CompileFunction(asIScriptModule *m, const char *sectionName, const char *code, int lineOffset, asDWORD compileFlags, asIScriptFunction **outFunc)               { return m->CompileFunction(sectionName, code, lineOffset, compileFlags, outFunc); }
	AS_API int                asModule_CompileGlobalVar(asIScriptModule *m, const char *sectionName, const char *code, int lineOffset)                                                                 { return m->CompileGlobalVar(sectionName, code, lineOffset); }
	AS_API asDWORD            asModule_SetAccessMask(asIScriptModule *m, asDWORD accessMask)                                                                                                        { return m->SetAccessMask(accessMask); }
	AS_API int                asModule_SetDefaultNamespace(asIScriptModule *m,const char *nameSpace)                                                                                               { return m->SetDefaultNamespace(nameSpace); };
	AS_API const char        *asModule_GetDefaultNamespace(asIScriptModule *m)                                                                                                                     { return m->GetDefaultNamespace(); }

	// Functions
	AS_API asUINT             asModule_GetFunctionCount(asIScriptModule *m)                                                                                                                            { return m->GetFunctionCount(); }
	AS_API asIScriptFunction *asModule_GetFunctionByIndex(asIScriptModule *m, asUINT index)                                                                                                            { return m->GetFunctionByIndex(index); }
	AS_API asIScriptFunction *asModule_GetFunctionByDecl(asIScriptModule *m, const char *decl)                                                                                                         { return m->GetFunctionByDecl(decl); }
	AS_API asIScriptFunction *asModule_GetFunctionByName(asIScriptModule *m, const char *name)                                                                                                         { return m->GetFunctionByName(name); } 
	AS_API int                asModule_RemoveFunction(asIScriptModule *m, asIScriptFunction *func)                                                                                                     { return m->RemoveFunction(func); }

	// Global variables
	AS_API int                asModule_ResetGlobalVars(asIScriptModule *m, asIScriptContext *ctx)                                                                                                                             { return m->ResetGlobalVars(ctx); }
	AS_API asUINT             asModule_GetGlobalVarCount(asIScriptModule *m)                                                                                                                           { return m->GetGlobalVarCount(); }
	AS_API int                asModule_GetGlobalVarIndexByName(asIScriptModule *m, const char *name)                                                                                                   { return m->GetGlobalVarIndexByName(name); }
	AS_API int                asModule_GetGlobalVarIndexByDecl(asIScriptModule *m, const char *decl)                                                                                                   { return m->GetGlobalVarIndexByDecl(decl); }
	AS_API const char        *asModule_GetGlobalVarDeclaration(asIScriptModule *m, asUINT index, asBOOL includeNamespace)                                                                                   { return m->GetGlobalVarDeclaration(index, includeNamespace ? true : false); }
	AS_API int                asModule_GetGlobalVar(asIScriptModule *m, asUINT index, const char **name, const char **nameSpace, int *typeId, asBOOL *isConst)                                           { bool _isConst; int r = m->GetGlobalVar(index, name, nameSpace, typeId, &_isConst); if( isConst ) *isConst = _isConst ? asTRUE : asFALSE; return r; }
	AS_API void              *asModule_GetAddressOfGlobalVar(asIScriptModule *m, asUINT index)                                                                                                            { return m->GetAddressOfGlobalVar(index); }
	AS_API int                asModule_RemoveGlobalVar(asIScriptModule *m, asUINT index)                                                                                                               { return m->RemoveGlobalVar(index); }

	// Type identification
	AS_API asUINT             asModule_GetObjectTypeCount(asIScriptModule *m)                                                   { return m->GetObjectTypeCount(); }
	AS_API asITypeInfo       *asModule_GetObjectTypeByIndex(asIScriptModule *m, asUINT index)                                   { return m->GetObjectTypeByIndex(index); }
	AS_API int                asModule_GetTypeIdByDecl(asIScriptModule *m, const char *decl)                                    { return m->GetTypeIdByDecl(decl); }
	AS_API asITypeInfo       *asModule_GetTypeInfoByName(asIScriptModule *m, const char *name) { return m->GetTypeInfoByName(name); }
	AS_API asITypeInfo       *asModule_GetTypeInfoByDecl(asIScriptModule *m, const char *decl) { return m->GetTypeInfoByDecl(decl); }

	// Enums
	AS_API asUINT             asModule_GetEnumCount(asIScriptModule *m)                                                         { return m->GetEnumCount(); }
	AS_API asITypeInfo *      asModule_GetEnumByIndex(asIScriptModule *m, asUINT index) { return m->GetEnumByIndex(index); }

	// Typedefs
	AS_API asUINT             asModule_GetTypedefCount(asIScriptModule *m)                                                      { return m->GetTypedefCount(); }
	AS_API asITypeInfo *      asModule_GetTypedefByIndex(asIScriptModule *m, asUINT index) { return m->GetTypedefByIndex(index); }

	// Dynamic binding between modules
	AS_API asUINT             asModule_GetImportedFunctionCount(asIScriptModule *m)                                             { return m->GetImportedFunctionCount(); }
	AS_API int                asModule_GetImportedFunctionIndexByDecl(asIScriptModule *m, const char *decl)                     { return m->GetImportedFunctionIndexByDecl(decl); }
	AS_API const char        *asModule_GetImportedFunctionDeclaration(asIScriptModule *m, asUINT importIndex)                    { return m->GetImportedFunctionDeclaration(importIndex); }
	AS_API const char        *asModule_GetImportedFunctionSourceModule(asIScriptModule *m, asUINT importIndex)                  { return m->GetImportedFunctionSourceModule(importIndex); }
	AS_API int                asModule_BindImportedFunction(asIScriptModule *m, asUINT importIndex, asIScriptFunction *func)    { return m->BindImportedFunction(importIndex, func); }
	AS_API int                asModule_UnbindImportedFunction(asIScriptModule *m, asUINT importIndex)                           { return m->UnbindImportedFunction(importIndex); }
	AS_API int                asModule_BindAllImportedFunctions(asIScriptModule *m)                                             { return m->BindAllImportedFunctions(); }
	AS_API int                asModule_UnbindAllImportedFunctions(asIScriptModule *m)                                           { return m->UnbindAllImportedFunctions(); } 

	// Bytecode saving and loading
	AS_API int                asModule_SaveByteCode(asIScriptModule *m, asIBinaryStream *out, asBOOL stripDebugInfo)              { return m->SaveByteCode(out, stripDebugInfo ? true : false); }
	AS_API int                asModule_LoadByteCode(asIScriptModule *m, asIBinaryStream *in, asBOOL *wasDebugInfoStripped)        { bool _wasDebugInfoStripped; int r = m->LoadByteCode(in, &_wasDebugInfoStripped); if( wasDebugInfoStripped ) *wasDebugInfoStripped = _wasDebugInfoStripped ? asTRUE : asFALSE; return r; }

	// User data
	AS_API void              *asModule_SetUserData(asIScriptModule *m, void *data)                                              { return m->SetUserData(data); }
	AS_API void              *asModule_GetUserData(asIScriptModule *m)                                                          { return m->GetUserData(); }

	///////////////////////////////////////////
	// asIScriptContext

	// Memory management
	AS_API int              asContext_AddRef(asIScriptContext *c)                                                               { return c->AddRef(); }
	AS_API int              asContext_Release(asIScriptContext *c)                                                              { return c->Release(); }

	// Miscellaneous
	AS_API asIScriptEngine *asContext_GetEngine(asIScriptContext *c)                                                            { return c->GetEngine(); }

	// Execution
	AS_API int              asContext_Prepare(asIScriptContext *c, asIScriptFunction *func)                                     { return c->Prepare(func); }
	AS_API int              asContext_Unprepare(asIScriptContext *c)                                                            { return c->Unprepare(); }
	AS_API int              asContext_Execute(asIScriptContext *c)                                                              { return c->Execute(); }
	AS_API int              asContext_Abort(asIScriptContext *c)                                                                { return c->Abort(); }
	AS_API int              asContext_Suspend(asIScriptContext *c)                                                              { return c->Suspend(); }
	AS_API asEContextState  asContext_GetState(asIScriptContext *c)                                                             { return c->GetState(); }
	AS_API int              asContext_PushState(asIScriptContext *c)                                                            { return c->PushState(); }
	AS_API int              asContext_PopState(asIScriptContext *c)                                                             { return c->PopState(); }
	AS_API asBOOL           asContext_IsNested(asIScriptContext *c, asUINT *nestCount)                                          { return c->IsNested(nestCount) ? asTRUE : asFALSE; }

	// Object pointer for calling class methods
	AS_API int              asContext_SetObject(asIScriptContext *c, void *obj)                                               { return c->SetObject(obj); }

	// Arguments
	AS_API int              asContext_SetArgByte(asIScriptContext *c, asUINT arg, asBYTE value)                                 { return c->SetArgByte(arg, value); }
	AS_API int              asContext_SetArgWord(asIScriptContext *c, asUINT arg, asWORD value)                                 { return c->SetArgWord(arg, value); }
	AS_API int              asContext_SetArgDWord(asIScriptContext *c, asUINT arg, asDWORD value)                               { return c->SetArgDWord(arg, value); }
	AS_API int              asContext_SetArgQWord(asIScriptContext *c, asUINT arg, asQWORD value)                               { return c->SetArgQWord(arg, value); }
	AS_API int              asContext_SetArgFloat(asIScriptContext *c, asUINT arg, float value)                                 { return c->SetArgFloat(arg, value); }
	AS_API int              asContext_SetArgDouble(asIScriptContext *c, asUINT arg, double value)                               { return c->SetArgDouble(arg, value); }
	AS_API int              asContext_SetArgAddress(asIScriptContext *c, asUINT arg, void *addr)                                { return c->SetArgAddress(arg, addr); }
	AS_API int              asContext_SetArgObject(asIScriptContext *c, asUINT arg, void *obj)                                  { return c->SetArgObject(arg, obj); }
	AS_API int              asContext_SetArgVarType(asIScriptContext *c, asUINT arg, void *ptr, int typeId) { return c->SetArgVarType(arg, ptr, typeId); }
	AS_API void *           asContext_GetAddressOfArg(asIScriptContext *c, asUINT arg)                                          { return c->GetAddressOfArg(arg); }

	// Return value
	AS_API asBYTE           asContext_GetReturnByte(asIScriptContext *c)                                                        { return c->GetReturnByte(); }
	AS_API asWORD           asContext_GetReturnWord(asIScriptContext *c)                                                        { return c->GetReturnWord(); }
	AS_API asDWORD          asContext_GetReturnDWord(asIScriptContext *c)                                                       { return c->GetReturnDWord(); }
	AS_API asQWORD          asContext_GetReturnQWord(asIScriptContext *c)                                                       { return c->GetReturnQWord(); }
	AS_API float            asContext_GetReturnFloat(asIScriptContext *c)                                                       { return c->GetReturnFloat(); }
	AS_API double           asContext_GetReturnDouble(asIScriptContext *c)                                                      { return c->GetReturnDouble(); }
	AS_API void *           asContext_GetReturnAddress(asIScriptContext *c)                                                     { return c->GetReturnAddress(); }
	AS_API void *           asContext_GetReturnObject(asIScriptContext *c)                                                      { return c->GetReturnObject(); }
	AS_API void *           asContext_GetAddressOfReturnValue(asIScriptContext *c)                                              { return c->GetAddressOfReturnValue(); }

	// Exception handling
	AS_API int                asContext_SetException(asIScriptContext *c, const char *string)                                     { return c->SetException(string); }
	AS_API int                asContext_GetExceptionLineNumber(asIScriptContext *c, int *column, const char **sectionName)                                  { return c->GetExceptionLineNumber(column, sectionName); }
	AS_API asIScriptFunction *asContext_GetExceptionFunction(asIScriptContext *c)                                              { return c->GetExceptionFunction(); }
	AS_API const char *       asContext_GetExceptionString(asIScriptContext *c)                                                   { return c->GetExceptionString(); }
	AS_API int                asContext_SetExceptionCallback(asIScriptContext *c, asFUNCTION_t callback, void *obj, int callConv) { return c->SetExceptionCallback(asFUNCTION(callback), obj, callConv); }
	AS_API void               asContext_ClearExceptionCallback(asIScriptContext *c)                                               { c->ClearExceptionCallback(); }

	// Debugging
	AS_API int                asContext_SetLineCallback(asIScriptContext *c, asFUNCTION_t callback, void *obj, int callConv)      { return c->SetLineCallback(asFUNCTION(callback), obj, callConv); }
	AS_API void               asContext_ClearLineCallback(asIScriptContext *c)                                                    { c->ClearLineCallback(); }
	AS_API asUINT             asContext_GetCallstackSize(asIScriptContext *c)                                                     { return c->GetCallstackSize(); }
	AS_API asIScriptFunction *asContext_GetFunction(asIScriptContext *c, asUINT stackLevel)                                     { return c->GetFunction(stackLevel); }
	AS_API int                asContext_GetLineNumber(asIScriptContext *c, asUINT stackLevel, int *column, const char **sectionName) { return c->GetLineNumber(stackLevel, column, sectionName); }
	AS_API int                asContext_GetVarCount(asIScriptContext *c, asUINT stackLevel)                                          { return c->GetVarCount(stackLevel); }
	AS_API const char *       asContext_GetVarName(asIScriptContext *c, asUINT varIndex, asUINT stackLevel)                             { return c->GetVarName(varIndex, stackLevel); }
	AS_API const char *       asContext_GetVarDeclaration(asIScriptContext *c, asUINT varIndex, asUINT stackLevel, asBOOL includeNamespace)                      { return c->GetVarDeclaration(varIndex, stackLevel, includeNamespace ? true : false); }
	AS_API int                asContext_GetVarTypeId(asIScriptContext *c, asUINT varIndex, asUINT stackLevel)                           { return c->GetVarTypeId(varIndex, stackLevel); }
	AS_API void *             asContext_GetAddressOfVar(asIScriptContext *c, asUINT varIndex, asUINT stackLevel)                        { return c->GetAddressOfVar(varIndex, stackLevel); }
	AS_API asBOOL             asContext_IsVarInScope(asIScriptContext *c, asUINT varIndex, asUINT stackLevel)                     { return c->IsVarInScope(varIndex, stackLevel) ? asTRUE : asFALSE; }
	AS_API int                asContext_GetThisTypeId(asIScriptContext *c, asUINT stackLevel)                                        { return c->GetThisTypeId(stackLevel); }
	AS_API void *             asContext_GetThisPointer(asIScriptContext *c, asUINT stackLevel)                                       { return c->GetThisPointer(stackLevel); }
	AS_API asIScriptFunction *asContext_GetSystemFunction(asIScriptContext *c)                                                  { return c->GetSystemFunction(); }

	// User data
	AS_API void *           asContext_SetUserData(asIScriptContext *c, void *data)                                              { return c->SetUserData(data); }
	AS_API void *           asContext_GetUserData(asIScriptContext *c)                                                          { return c->GetUserData(); }

	///////////////////////////////////////////
	// asIScriptGeneric

	// Miscellaneous
	AS_API asIScriptEngine   *asGeneric_GetEngine(asIScriptGeneric *g)                  { return g->GetEngine(); }
	AS_API asIScriptFunction *asGeneric_GetFunction(asIScriptGeneric *g)                { return g->GetFunction(); }
	AS_API void              *asGeneric_GetAuxiliary(asIScriptGeneric *g) { return g->GetAuxiliary(); }

	// Object
	AS_API void *           asGeneric_GetObject(asIScriptGeneric *g)                    { return g->GetObject(); }
	AS_API int              asGeneric_GetObjectTypeId(asIScriptGeneric *g)              { return g->GetObjectTypeId(); }

	// Arguments
	AS_API int              asGeneric_GetArgCount(asIScriptGeneric *g)                  { return g->GetArgCount(); }
	AS_API int              asGeneric_GetArgTypeId(asIScriptGeneric *g, asUINT arg, asDWORD *flags)     { return g->GetArgTypeId(arg, flags); }
	AS_API asBYTE           asGeneric_GetArgByte(asIScriptGeneric *g, asUINT arg)       { return g->GetArgByte(arg); }
	AS_API asWORD           asGeneric_GetArgWord(asIScriptGeneric *g, asUINT arg)       { return g->GetArgWord(arg); }
	AS_API asDWORD          asGeneric_GetArgDWord(asIScriptGeneric *g, asUINT arg)      { return g->GetArgDWord(arg); }
	AS_API asQWORD          asGeneric_GetArgQWord(asIScriptGeneric *g, asUINT arg)      { return g->GetArgQWord(arg); }
	AS_API float            asGeneric_GetArgFloat(asIScriptGeneric *g, asUINT arg)      { return g->GetArgFloat(arg); }
	AS_API double           asGeneric_GetArgDouble(asIScriptGeneric *g, asUINT arg)     { return g->GetArgDouble(arg); }
	AS_API void *           asGeneric_GetArgAddress(asIScriptGeneric *g, asUINT arg)    { return g->GetArgAddress(arg); }
	AS_API void *           asGeneric_GetArgObject(asIScriptGeneric *g, asUINT arg)     { return g->GetArgObject(arg); }
	AS_API void *           asGeneric_GetAddressOfArg(asIScriptGeneric *g, asUINT arg)  { return g->GetAddressOfArg(arg); }

	// Return value
	AS_API int              asGeneric_GetReturnTypeId(asIScriptGeneric *g, asDWORD *flags)              { return g->GetReturnTypeId(flags); }
	AS_API int              asGeneric_SetReturnByte(asIScriptGeneric *g, asBYTE val)    { return g->SetReturnByte(val); }
	AS_API int              asGeneric_SetReturnWord(asIScriptGeneric *g, asWORD val)    { return g->SetReturnWord(val); }
	AS_API int              asGeneric_SetReturnDWord(asIScriptGeneric *g, asDWORD val)  { return g->SetReturnDWord(val); }
	AS_API int              asGeneric_SetReturnQWord(asIScriptGeneric *g, asQWORD val)  { return g->SetReturnQWord(val); }
	AS_API int              asGeneric_SetReturnFloat(asIScriptGeneric *g, float val)    { return g->SetReturnFloat(val); }
	AS_API int              asGeneric_SetReturnDouble(asIScriptGeneric *g, double val)  { return g->SetReturnDouble(val); }
	AS_API int              asGeneric_SetReturnAddress(asIScriptGeneric *g, void *addr) { return g->SetReturnAddress(addr); }
	AS_API int              asGeneric_SetReturnObject(asIScriptGeneric *g, void *obj)   { return g->SetReturnObject(obj); }
	AS_API void *           asGeneric_GetAddressOfReturnLocation(asIScriptGeneric *g)   { return g->GetAddressOfReturnLocation(); }

	///////////////////////////////////////////
	// asIScriptObject

	// Memory management
	AS_API int              asObject_AddRef(asIScriptObject *s)                           { return s->AddRef(); }
	AS_API int              asObject_Release(asIScriptObject *s)                          { return s->Release(); }
	AS_API asILockableSharedBool *asObject_GetWeakRefFlag(asIScriptObject *s) { return s->GetWeakRefFlag(); }

	// Type info
	AS_API int              asObject_GetTypeId(asIScriptObject *s)                        { return s->GetTypeId(); }
	AS_API asITypeInfo *    asObject_GetObjectType(asIScriptObject *s)                    { return s->GetObjectType(); }

	// Class properties
	AS_API asUINT           asObject_GetPropertyCount(asIScriptObject *s)                 { return s->GetPropertyCount(); }
	AS_API int              asObject_GetPropertyTypeId(asIScriptObject *s, asUINT prop)   { return s->GetPropertyTypeId(prop); }
	AS_API const char *     asObject_GetPropertyName(asIScriptObject *s, asUINT prop)     { return s->GetPropertyName(prop); }
	AS_API void *           asObject_GetAddressOfProperty(asIScriptObject *s, asUINT prop){ return s->GetAddressOfProperty(prop); }

	// Miscellaneous
	AS_API asIScriptEngine *asObject_GetEngine(asIScriptObject *s)                        { return s->GetEngine(); }
	AS_API int              asObject_CopyFrom(asIScriptObject *s, asIScriptObject *other) { return s->CopyFrom(other); }

	// User data
	AS_API void *asObject_SetUserData(asIScriptObject *s, void *data, asPWORD type) { return s->SetUserData(data, type); }
	AS_API void *asObject_GetUserData(asIScriptObject *s, asPWORD type) { return s->GetUserData(type); }

	///////////////////////////////////////////
	// asITypeInfo

	// Miscellaneous
	AS_API asIScriptEngine *asTypeInfo_GetEngine(const asITypeInfo *o)                                    { return o->GetEngine(); }
	AS_API const char *     asTypeInfo_GetConfigGroup(const asITypeInfo *o)                               { return o->GetConfigGroup(); }
	AS_API asDWORD          asTypeInfo_GetAccessMask(const asITypeInfo *o)                                { return o->GetAccessMask(); }
	AS_API asIScriptModule *asTypeInfo_GetModule(asITypeInfo *o) { return o->GetModule(); }

	// Memory management
	AS_API int              asTypeInfo_AddRef(const asITypeInfo *o)                                           { return o->AddRef(); }
	AS_API int              asTypeInfo_Release(const asITypeInfo *o)                                          { return o->Release(); }

	// Type info
	AS_API const char *     asTypeInfo_GetName(const asITypeInfo *o)                                      { return o->GetName(); }
	AS_API const char *     asTypeInfo_GetNamespace(const asITypeInfo *o)                                 { return o->GetNamespace(); }
	AS_API asITypeInfo *    asTypeInfo_GetBaseType(const asITypeInfo *o)                                  { return o->GetBaseType(); }
	AS_API asBOOL           asTypeInfo_DerivesFrom(const asITypeInfo *o, const asITypeInfo *objType)      { return o->DerivesFrom(objType) ? asTRUE : asFALSE; }
	AS_API asDWORD          asTypeInfo_GetFlags(const asITypeInfo *o)                                     { return o->GetFlags(); }
	AS_API asUINT           asTypeInfo_GetSize(const asITypeInfo *o)                                      { return o->GetSize(); }
	AS_API int              asTypeInfo_GetTypeId(const asITypeInfo *o)                                    { return o->GetTypeId(); }
	AS_API int              asTypeInfo_GetSubTypeId(const asITypeInfo *o, asUINT subTypeIndex)            { return o->GetSubTypeId(subTypeIndex); }
	AS_API asITypeInfo *    asTypeInfo_GetSubType(const asITypeInfo *o, asUINT subTypeIndex)              { return o->GetSubType(subTypeIndex); }
	AS_API asUINT           asTypeInfo_GetSubTypeCount(const asITypeInfo *o)                              { return o->GetSubTypeCount(); }

	// Interfaces
	AS_API asUINT           asTypeInfo_GetInterfaceCount(const asITypeInfo *o)                            { return o->GetInterfaceCount(); }
	AS_API asITypeInfo *    asTypeInfo_GetInterface(const asITypeInfo *o, asUINT index)                   { return o->GetInterface(index); }
	AS_API asBOOL           asTypeInfo_Implements(const asITypeInfo *o, const asITypeInfo *objType)       { return o->Implements(objType) ? asTRUE : asFALSE; }

	// Factories
	AS_API asUINT             asTypeInfo_GetFactoryCount(const asITypeInfo *o)                                      { return o->GetFactoryCount(); }
	AS_API asIScriptFunction *asTypeInfo_GetFactoryByIndex(const asITypeInfo *o, asUINT index)                      { return o->GetFactoryByIndex(index); }
	AS_API asIScriptFunction *asTypeInfo_GetFactoryByDecl(const asITypeInfo *o, const char *decl)                   { return o->GetFactoryByDecl(decl); }

	// Methods
	AS_API asUINT             asTypeInfo_GetMethodCount(const asITypeInfo *o)                                         { return o->GetMethodCount(); }
	AS_API asIScriptFunction *asTypeInfo_GetMethodByIndex(const asITypeInfo *o, asUINT index, asBOOL getVirtual)      { return o->GetMethodByIndex(index, getVirtual ? true : false); }
	AS_API asIScriptFunction *asTypeInfo_GetMethodByName(const asITypeInfo *o, const char *name, asBOOL getVirtual)   { return o->GetMethodByName(name, getVirtual ? true : false); }
	AS_API asIScriptFunction *asTypeInfo_GetMethodByDecl(const asITypeInfo *o, const char *decl, asBOOL getVirtual)   { return o->GetMethodByDecl(decl, getVirtual ? true : false); }

	// Properties
	AS_API asUINT      asTypeInfo_GetPropertyCount(const asITypeInfo *o)                                            { return o->GetPropertyCount(); }
	AS_API int         asTypeInfo_GetProperty(const asITypeInfo *o, asUINT index, const char **name, int *typeId, asBOOL *isPrivate, asBOOL *isProtected, int *offset, asBOOL *isReference, asDWORD *accessMask) { bool _isPrivate, _isProtected, _isReference; int r = o->GetProperty(index, name, typeId, &_isPrivate, &_isProtected, offset, &_isReference, accessMask); if (isPrivate) *isPrivate = _isPrivate ? asTRUE : asFALSE; if (isProtected) *isProtected = _isProtected ? asTRUE : asFALSE; if (isReference) *isReference = _isReference ? asTRUE : asFALSE; return r; }
	AS_API const char *asTypeInfo_GetPropertyDeclaration(const asITypeInfo *o, asUINT index)                        { return o->GetPropertyDeclaration(index); }

	// Behaviours
	AS_API asUINT             asTypeInfo_GetBehaviourCount(const asITypeInfo *o)                                              { return o->GetBehaviourCount(); }
	AS_API asIScriptFunction *asTypeInfo_GetBehaviourByIndex(const asITypeInfo *o, asUINT index, asEBehaviours *outBehaviour) { return o->GetBehaviourByIndex(index, outBehaviour); }

	// Child types
	AS_API asUINT       asTypeInfo_GetChildFuncdefCount(asITypeInfo *o) { return o->GetChildFuncdefCount(); }
	AS_API asITypeInfo *asTypeInfo_GetChildFuncdef(asITypeInfo *o, asUINT index) { return o->GetChildFuncdef(index); }
	AS_API asITypeInfo *asTypeInfo_GetParentType(asITypeInfo *o) { return o->GetParentType(); }

	// Enums
	AS_API asUINT      asTypeInfo_GetEnumValueCount(asITypeInfo *o) { return o->GetEnumValueCount(); }
	AS_API const char *asTypeInfo_GetEnumValueByIndex(asITypeInfo *o, asUINT index, int *outValue) { return o->GetEnumValueByIndex(index, outValue); }

	// Typedef
	AS_API int asTypeInfo_GetTypedefTypeId(asITypeInfo *o) { return o->GetTypedefTypeId(); }

	// Funcdef
	AS_API asIScriptFunction *asTypeInfo_GetFuncdefSignature(asITypeInfo *o) { return o->GetFuncdefSignature(); }

	// User data
	AS_API void            *asTypeInfo_SetUserData(asITypeInfo *o, void *data, asPWORD type)                        { return o->SetUserData(data, type); }
	AS_API void            *asTypeInfo_GetUserData(asITypeInfo *o, asPWORD type)                                    { return o->GetUserData(type); }

	///////////////////////////////////////////
	// asIScriptFunction

	AS_API asIScriptEngine *asFunction_GetEngine(const asIScriptFunction *f)                                                     { return f->GetEngine(); }

	// Memory management
	AS_API int              asFunction_AddRef(const asIScriptFunction *f)                                                        { return f->AddRef(); }
	AS_API int              asFunction_Release(const asIScriptFunction *f)                                                       { return f->Release(); }

	// Miscellaneous
	AS_API int              asFunction_GetId(const asIScriptFunction *f)                                                         { return f->GetId(); }
	AS_API asEFuncType      asFunction_GetFuncType(const asIScriptFunction *f)                                                   { return f->GetFuncType(); }
	AS_API const char      *asFunction_GetModuleName(const asIScriptFunction *f)                                                 { return f->GetModuleName(); }
	AS_API asIScriptModule *asFunction_GetModule(const asIScriptFunction *f) { return f->GetModule(); }
	AS_API const char      *asFunction_GetScriptSectionName(const asIScriptFunction *f)                                          { return f->GetScriptSectionName(); }
	AS_API const char      *asFunction_GetConfigGroup(const asIScriptFunction *f)                                                { return f->GetConfigGroup(); }
	AS_API asDWORD          asFunction_GetAccessMask(const asIScriptFunction *f)                                               { return f->GetAccessMask(); }
	AS_API void            *asFunction_GetAuxiliary(const asIScriptFunction *f) { return f->GetAuxiliary(); }

	// Function signature
	AS_API asITypeInfo     *asFunction_GetObjectType(const asIScriptFunction *f)                                                 { return f->GetObjectType(); }
	AS_API const char      *asFunction_GetObjectName(const asIScriptFunction *f)                                                 { return f->GetObjectName(); }
	AS_API const char      *asFunction_GetName(const asIScriptFunction *f)                                                       { return f->GetName(); }
	AS_API const char      *asFunction_GetNamespace(const asIScriptFunction *f)                                                  { return f->GetNamespace(); }
	AS_API const char      *asFunction_GetDeclaration(const asIScriptFunction *f, asBOOL includeObjectName, asBOOL includeNamespace) { return f->GetDeclaration(includeObjectName ? true : false, includeNamespace ? true : false); }
	AS_API asBOOL           asFunction_IsReadOnly(const asIScriptFunction *f)                                                    { return f->IsReadOnly() ? asTRUE : asFALSE; }
	AS_API asBOOL           asFunction_IsPrivate(const asIScriptFunction *f)                                                     { return f->IsPrivate() ? asTRUE : asFALSE; }
	AS_API asBOOL           asFunction_IsProtected(const asIScriptFunction *f) { return f->IsProtected() ? asTRUE : asFALSE; }
	AS_API asBOOL           asFunction_IsFinal(const asIScriptFunction *f)                                                       { return f->IsFinal() ? asTRUE : asFALSE; }
	AS_API asBOOL           asFunction_IsOverride(const asIScriptFunction *f)                                                    { return f->IsOverride() ? asTRUE : asFALSE; }
	AS_API asBOOL           asFunction_IsShared(const asIScriptFunction *f)                                                      { return f->IsShared() ? asTRUE : asFALSE; }
	AS_API asUINT           asFunction_GetParamCount(const asIScriptFunction *f)                                                 { return f->GetParamCount(); }
	AS_API int              asFunction_GetParam(const asIScriptFunction *f, asUINT index, int *typeId, asDWORD *flags, const char **name, const char **defaultArg) { return f->GetParam(index, typeId, flags, name, defaultArg); }
	AS_API int              asFunction_GetReturnTypeId(const asIScriptFunction *f)                                               { return f->GetReturnTypeId(); }

	// Type id for function pointers
	AS_API int              asFunction_GetTypeId(const asIScriptFunction *f)                                                     { return f->GetTypeId(); }
	AS_API asBOOL           asFunction_IsCompatibleWithTypeId(const asIScriptFunction *f, int typeId)                            { return f->IsCompatibleWithTypeId(typeId) ? asTRUE : asFALSE; }

	// Delegates
	AS_API void              *asFunction_GetDelegateObject(const asIScriptFunction *f) { return f->GetDelegateObject(); }
	AS_API asITypeInfo       *asFunction_GetDelegateObjectType(const asIScriptFunction *f) { return f->GetDelegateObjectType(); }
	AS_API asIScriptFunction *asFunction_GetDelegateFunction(const asIScriptFunction *f) { return f->GetDelegateFunction(); }

	// Debug information
	AS_API asUINT           asFunction_GetVarCount(const asIScriptFunction *f)                                                   { return f->GetVarCount(); }
	AS_API int              asFunction_GetVar(const asIScriptFunction *f, asUINT index, const char **name, int *typeId)          { return f->GetVar(index, name, typeId); }
	AS_API const char *     asFunction_GetVarDecl(const asIScriptFunction *f, asUINT index, asBOOL includeNamespace)                                      { return f->GetVarDecl(index, includeNamespace ? true : false); }
	AS_API int              asFunction_FindNextLineWithCode(const asIScriptFunction *f, int line)                                { return f->FindNextLineWithCode(line); }

	// For JIT compilation
	AS_API asDWORD         *asFunction_GetByteCode(asIScriptFunction *f, asUINT *length)                                         { return f->GetByteCode(length); }

	// User data
	AS_API void            *asFunction_SetUserData(asIScriptFunction *f, void *userData)                                         { return f->SetUserData(userData); }
	AS_API void            *asFunction_GetUserData(const asIScriptFunction *f)                                                   { return f->GetUserData(); }

}

END_AS_NAMESPACE



