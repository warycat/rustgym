use crate::ControlManager;

#[derive(Default)]
pub struct RewindManager {}

impl RewindManager {
    pub fn new() -> Self {
        let this = RewindManager::default();
        this
    }
}
