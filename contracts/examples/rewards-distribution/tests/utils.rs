use elrond_wasm::types::ManagedVec;
use elrond_wasm_debug::DebugApi;
use rewards_distribution::Bracket;

pub fn to_brackets(brackets_vec: &[(u64, u64)]) -> ManagedVec<DebugApi, Bracket> {
    let mut brackets = ManagedVec::<DebugApi, Bracket>::new();
    for (index_percent, reward_percent) in brackets_vec.iter().cloned() {
        brackets.push(Bracket {
            index_percent,
            reward_percent,
        });
    }
    brackets
}
