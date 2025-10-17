// required kien_tanh in Le. Both platform and the service can have both outer-agent
// and inner_agent. The independent service can subscribe to these agents to accumulate info
pub fn le_outer_agent() -> String {
    return "le_outer_agent_platform_message".to_string();
}    

pub fn le_inner_agent() -> String {
    return "le_inner_agent_service_message".to_string();
} 

