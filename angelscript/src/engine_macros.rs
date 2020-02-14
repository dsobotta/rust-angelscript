//generic macro for use by debug/warning/error variants
#[macro_export]
macro_rules! as_send_message {
    (
        $engine: expr,
        $msg_type: expr,
        $message: expr
    ) => {
        //TODO: do something with result
        let _result = angelscript::engine::ScriptEngine::send_message(
            &mut $engine, 
            file!(), line!(), column!(), 
            $msg_type, 
            $message);
    };
}

#[macro_export]
macro_rules! as_log_debug {
    ( $engine: expr, $message: expr ) => {
        angelscript::as_send_message!($engine, angelscript_sys::c_types::asEMsgType_asMSGTYPE_INFORMATION, $message);
    }
}

#[macro_export]
macro_rules! as_log_warning {
    ( $engine: expr, $message: expr ) => {
        angelscript::as_send_message!($engine, angelscript_sys::c_types::asEMsgType_asMSGTYPE_WARNING, $message);
    }
}

#[macro_export]
macro_rules! as_log_error {
    ( $engine: expr, $message: expr ) => {
        angelscript::as_send_message!($engine, angelscript_sys::c_types::asEMsgType_asMSGTYPE_ERROR, $message);
    }
}

#[macro_export]
macro_rules! check_ok {
    ( $ret_code: expr ) => {
        let code: angelscript::types::EReturnCodes = $ret_code;
        match code {
            angelscript::types::EReturnCodes::Success => (),
            _ => panic!("AngelScript failure!  r = {}", code as i32)
        }
    }
}