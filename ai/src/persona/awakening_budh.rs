// required kien_tanh in AwakeningBudh. Both platform and the service can have both outer-agent
// and inner_agent. The independent service can subscribe to these agents to accumulate info
pub fn awakening_budh_outer_agent() -> String {
    return "awakening_budh_outer_agent_platform_message".to_string();
}    

pub fn awakening_budh_inner_agent() -> String {
    return "awkening_budh_inner_agent_service_message".to_string();
} 

