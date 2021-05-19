use log::{info, LevelFilter};
use open62541_sys::server::*;

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, LevelFilter::Debug)
        .init();

    info!("starting open62541 sample server");
    let server = unsafe { UA_Server_new() };

    let config = unsafe { UA_Server_getConfig(server) };
    let status =
        unsafe { UA_ServerConfig_setMinimalCustomBuffer(config, 4840, &UA_STRING_NULL, 0, 0) };
    info!("config: {}", status);
    unsafe { (*config).verifyRequestTimestamp = UA_RuleHandling_UA_RULEHANDLING_ACCEPT };

    add_var(server, "half", 21);
    add_var(server, "answer", 42);

    let running = true;
    let status = unsafe { UA_Server_run(server, &running) };
    info!("run: {}", status);
}

fn add_var(server: *mut UA_Server, name: &str, val: i32) {
    // static void
    // addVariable(UA_Server *server) {

    //     /* Define the attribute of the myInteger variable node */
    //     UA_VariableAttributes attr = UA_VariableAttributes_default;
    //     UA_Int32 myInteger = 42;
    //     UA_Variant_setScalar(&attr.value, &myInteger, &UA_TYPES[UA_TYPES_INT32]);
    //     attr.description = UA_LOCALIZEDTEXT("en-US","the answer");
    //     attr.displayName = UA_LOCALIZEDTEXT("en-US","the answer");
    //     attr.dataType = UA_TYPES[UA_TYPES_INT32].typeId;
    //     attr.accessLevel = UA_ACCESSLEVELMASK_READ | UA_ACCESSLEVELMASK_WRITE;

    let mut attr = unsafe { Box::new(UA_VariableAttributes_default) };

    let my_integer = Box::new(val);
    let my_integer_ptr = Box::into_raw(my_integer) as *mut std::os::raw::c_void;

    let attr_ptr = &mut attr.value as *mut UA_Variant;
    unsafe {
        UA_Variant_setScalarCopy(attr_ptr, my_integer_ptr, &UA_TYPES[UA_TYPES_INT32 as usize])
    };

    let loc_descr = UA_LocalizedText {
        locale: UA_String::from("en-en"),
        text: UA_String::from("The Answer"),
    };
    attr.description = loc_descr;
    attr.displayName = loc_descr;
    attr.dataType = unsafe { UA_TYPES[UA_TYPES_INT32 as usize].typeId };
    attr.accessLevel = UA_ACCESSLEVELMASK_READ as u8;

    //     /* Add the variable node to the information model */
    //     UA_NodeId myIntegerNodeId = UA_NODEID_STRING(1, "the.answer");
    //     UA_QualifiedName myIntegerName = UA_QUALIFIEDNAME(1, "the answer");
    //     UA_NodeId parentNodeId = UA_NODEID_NUMERIC(0, UA_NS0ID_OBJECTSFOLDER);
    //     UA_NodeId parentReferenceNodeId = UA_NODEID_NUMERIC(0, UA_NS0ID_ORGANIZES);
    //     UA_Server_addVariableNode(server, myIntegerNodeId, parentNodeId,
    //                               parentReferenceNodeId, myIntegerName,
    //                               UA_NODEID_NUMERIC(0, UA_NS0ID_BASEDATAVARIABLETYPE), attr, NULL, NULL);
    // }

    let my_integer_name = UA_QualifiedName {
        namespaceIndex: 1,
        name: UA_String::from("the.answer"),
    };
    let type_id = UA_NodeId::from(format!("ns=0;i={}", UA_NS0ID_BASEDATAVARIABLETYPE));
    let req_node_id = UA_NodeId::from(format!("ns=1;s={}", name));
    let parent_id = UA_NodeId::from(format!("ns=0;i={}", UA_NS0ID_OBJECTSFOLDER));
    let parent_ref_id = UA_NodeId::from(format!("ns=0;i={}", UA_NS0ID_ORGANIZES));

    let attr_ptr = Box::into_raw(attr) as *mut std::os::raw::c_void;

    let node_context = Box::new(0);
    let node_context_ptr = Box::into_raw(node_context) as *mut std::os::raw::c_void;

    let mut node_id = unsafe { UA_NODEID_NULL };
    let node_id_ptr = &mut node_id as *mut UA_NodeId;

    let status = unsafe {
        UA_Server_addNode_begin(
            server,
            UA_NodeClass_UA_NODECLASS_VARIABLE,
            req_node_id,
            parent_id,
            parent_ref_id,
            my_integer_name,
            type_id,
            attr_ptr,
            &(UA_TYPES[UA_TYPES_VARIABLEATTRIBUTES as usize]),
            node_context_ptr,
            node_id_ptr,
        )
    };
    info!("add node begin: {}", status);

    let status = unsafe { UA_Server_addNode_finish(server, node_id) };
    info!("add node finish: {} - {}", status, node_id);
}
