#include <open62541/architecture_definitions.h>

#ifdef UA_STATIC_INLINE
#   undef UA_STATIC_INLINE
#   define UA_STATIC_INLINE extern inline
#endif

#include <open62541/server.h>
#include <open62541/server_config_default.h>
