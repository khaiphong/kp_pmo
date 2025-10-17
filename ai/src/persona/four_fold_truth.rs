// required kien_tanh in FourFoldTruth. Both platform and the service can have both outer-agent
// and inner_agent. The independent service can subscribe to these agents to accumulate info
pub fn four_fold_truth_outer_agent() -> String {
    return "four_fold_truth_outer_agent_platform_message".to_string();
}    

pub fn four_fold_truth_inner_agent() -> String {
    return "four_fold_truth_inner_agent_service_message".to_string();
} 

