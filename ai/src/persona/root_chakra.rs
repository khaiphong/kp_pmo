// required kien_tanh in RootChakra. Both platform and the service can have both outer-agent
// and inner_agent. The independent service can subscribe to these agents to accumulate info
pub fn root_chakra_outer_agent() -> String {
    return "root_chakra_outer_agent_platform_message".to_string();
}    

pub fn root_chakra_inner_agent() -> String {
    return "root_chakra_inner_agent_service_message".to_string();
} 

