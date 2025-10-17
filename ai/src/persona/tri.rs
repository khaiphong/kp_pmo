// required kien_tanh in Tri. Both platform and the service can have both outer-agent
// and inner_agent. The independent service can subscribe to these agents to accumulate info
pub fn heaven_outer_agent() -> String {
    return "tri_outer_agent_platform_message".to_string();
}    

pub fn tri_inner_agent() -> String {
    return "tri_inner_agent_service_message".to_string();
} 

