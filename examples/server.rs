use log::{info, LevelFilter};
use open62541_sys::*;

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, LevelFilter::Debug)
        .init();

    info!("starting open62541 sample server");
    let server = unsafe { server::UA_Server_new() };

    let config = unsafe { server::UA_Server_getConfig(server) };
    let status = unsafe {
        server::UA_ServerConfig_setMinimalCustomBuffer(config, 4840, &server::UA_STRING_NULL, 0, 0)
    };
    println!("config: {}", status);
    unsafe { (*config).verifyRequestTimestamp = server::UA_RuleHandling_UA_RULEHANDLING_ACCEPT };

    let running = true;
    let _status = unsafe { server::UA_Server_run(server, &running) };
}
