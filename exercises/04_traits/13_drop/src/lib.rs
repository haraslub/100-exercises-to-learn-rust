// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

enum BombState {
    actived,
    defused,
}

impl BombState {
    fn is_defused(&self) -> bool {
        match self {
            BombState::actived => false,
            BombState::defused => true
        }
    }
}

pub struct DropBomb {
    state: BombState,
}

impl DropBomb {
    pub fn new() -> DropBomb {
        DropBomb {
            state: BombState::actived,
        }
    }

    pub fn defuse(&mut self) {
        self.state = BombState::defused;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.state.is_defused() {
            panic!("Bomb is activated")
        }
        // match self.state {
        //     BombState::actived => panic!("Bomb is activated"),
        //     BombState::defused => ()
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
