use crate::server::*;
use std::fmt::Display;

impl From<&str> for UA_String {
    fn from(val: &str) -> Self {
        let cs = std::ffi::CString::new(val).unwrap();
        unsafe { UA_String_fromChars(cs.as_ptr()) }
    }
}

impl From<String> for UA_String {
    fn from(val: String) -> Self {
        let cs = std::ffi::CString::new(val).unwrap();
        unsafe { UA_String_fromChars(cs.as_ptr()) }
    }
}

impl From<&UA_String> for String {
    fn from(val: &UA_String) -> Self {
        let v: Vec<u8> =
            unsafe { Vec::from_raw_parts(val.data, val.length as usize, val.length as usize) };
        String::from_utf8(v).unwrap()
    }
}

impl Display for UA_String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from(self);
        Display::fmt(&s, f)
    }
}

impl From<&str> for UA_NodeId {
    fn from(val: &str) -> Self {
        let mut node_id = unsafe { UA_NODEID_NULL };
        let node_id_ptr = &mut node_id as *mut UA_NodeId;
        let ua_str = UA_String::from(val);
        unsafe { UA_NodeId_parse(node_id_ptr, ua_str) };
        node_id
    }
}

impl From<String> for UA_NodeId {
    fn from(val: String) -> Self {
        let mut node_id = unsafe { UA_NODEID_NULL };
        let node_id_ptr = &mut node_id as *mut UA_NodeId;
        let ua_str = UA_String::from(val);
        unsafe { UA_NodeId_parse(node_id_ptr, ua_str) };
        node_id
    }
}

impl From<&UA_NodeId> for String {
    fn from(val: &UA_NodeId) -> Self {
        let s = Box::new(UA_String::from(""));
        let s_ptr = Box::into_raw(s);
        unsafe { UA_NodeId_print(val, s_ptr) };

        let s = unsafe { Box::from_raw(s_ptr) };
        String::from(s.as_ref())
    }
}

impl Display for UA_NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from(self);
        Display::fmt(&s, f)
    }
}
