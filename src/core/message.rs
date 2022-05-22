use bincode::{Encode, Decode};
use crate::core::message::Message::Propose;

#[derive(Encode, Decode)]
pub struct Prepare {
    proposal_id: i32,
    from: i32,
}

impl Prepare {
    pub fn new(proposal_id: i32, from: i32) -> Self {
        Prepare{proposal_id, from}
    }
}

#[derive(Encode, Decode)]
pub struct Promise {
    proposal_id: i32,
    accepted_proposal_id: Option<i32>,
    accepted_proposal_value: Option<i32>,
}

impl Promise {
    pub fn new_with_proposal_id(proposal_id: i32) -> Self {
        Promise{proposal_id, accepted_proposal_id: None, accepted_proposal_value: None}
    }

    pub fn new(proposal_id: i32, accepted_proposal_id: i32, accepted_proposal_value: i32) -> Self {
        Promise{proposal_id, accepted_proposal_id: Option::Some(accepted_proposal_id), accepted_proposal_value: Option::Some(accepted_proposal_value)}
    }
}

#[derive(Encode, Decode)]
pub struct Propose {
    proposal_id: int,
    proposal_value: int,
    from: int,
}

#[derive(Encode, Decode)]
pub struct Accepted {
    proposal_id: int,
    proposal_value: int,
}

#[derive(Encode, Decode)]
pub enum Message {
    Prepare(Prepare),
    Promise(Promise),
    Propose(Propose),
    Accepted(Accepted),
}