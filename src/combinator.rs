/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/nimble-rust/host
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use std::collections::HashMap;

use nimble_steps::{ParticipantSteps, Steps};

pub struct Combinator<T> {
    pub in_buffers: HashMap<u8, Steps<T>>,
}

impl<T> Combinator<T> {
    pub fn new() -> Self {
        Combinator {
            in_buffers: HashMap::new(),
        }
    }

    pub fn create_buffer(&mut self, id: u8) {
        self.in_buffers.insert(id, Steps::new());
    }

    pub fn add(&mut self, id: u8, steps: ParticipantSteps<T>) {
        if let Some(buffer) = self.in_buffers.get_mut(&id) {
            buffer.push(steps);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestStep {
        Ingame(i8),
        SelectTeam(u16),
    }

    #[test]
    fn test_combinator_add() {
        let mut combinator = Combinator::<TestStep>::new();
        combinator.create_buffer(1);
        combinator.create_buffer(2);

        let mut steps1 = ParticipantSteps::new();
        steps1.push(8, nimble_steps::Step::Custom(TestStep::Ingame(-2)));
        let mut steps2 = ParticipantSteps::new();
        steps2.push(8, nimble_steps::Step::Custom(TestStep::SelectTeam(42)));

        combinator.add(1, steps1);
        combinator.add(2, steps2);

        assert_eq!(combinator.in_buffers.len(), 2);
        assert_eq!(combinator.in_buffers.get(&1).unwrap().len(), 1);
        assert_eq!(combinator.in_buffers.get(&2).unwrap().len(), 1);
    }
}
