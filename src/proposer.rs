use crate::core::message::*;

struct Proposer<> {
    id: int,
    round: int,
    number: int,
    acceptors: Vec<i32>,
}

impl Proposer {
    pub fn new(id: int, acceptors: &[i32]) -> Self {
        Proposer{ id, round: 0, number: 0, acceptors: acceptors.to_vec()};
    }

    pub fn propose(&mut self, proposal_id: int) {
        self.inc_round();
        self.update_number();

        for acceptor in self.acceptors {
            let mut prepare = Prepare{proposal_id, from: self.id };
        }
    }

    fn inc_round(&mut self) {
        self.round += 1;
    }

    fn update_number(&mut self) {
        self.number;
    }
}