use multiversx_sc::types::ManagedVec;
use multiversx_sc_scenario::DebugApi;
use rewards_distribution::Bracket;

pub fn to_brackets(brackets_vec: &[(u64, u64)]) -> ManagedVec<DebugApi, Bracket> {
    let mut brackets = ManagedVec::<DebugApi, Bracket>::new();
    for (index_percent, bracket_reward_percent) in brackets_vec.iter().cloned() {
        brackets.push(Bracket {
            index_percent,
            bracket_reward_percent,
        });
    }
    brackets
}
