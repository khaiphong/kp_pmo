// required kien_tanh in ThirdEye. Both platform and the service can have both outer-agent
// and inner_agent. The independent service can subscribe to these agents to accumulate info
pub fn third_eye_outer_agent() -> String {
    return "third_eye_outer_agent_platform_message".to_string();
}    

pub fn third_eye_inner_agent() -> String {
    return "third_eye_inner_agent_service_message".to_string();
} 

