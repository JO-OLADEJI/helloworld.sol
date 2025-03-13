use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingState {
    pub current: String,
    pub exp_time: u64,
}
