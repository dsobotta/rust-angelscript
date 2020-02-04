use crate::angelscript::types::*;
use crate::angelscript::engine_c::*;

pub struct ScriptEngine {
    engine: *mut asIScriptEngine
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        return ScriptEngine {
            engine: unsafe { asCreateScriptEngine(ANGELSCRIPT_VERSION) }
        }
    }

    pub fn get_global_function_count(&self) -> u32 {
        let count = unsafe { asEngine_GetGlobalFunctionCount(self.engine) };
        return count as u32;
    }
}