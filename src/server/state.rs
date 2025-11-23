/// Server state representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerState {
    Playing,
    Stopped,
    Interactive,
}

impl ServerState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ServerState::Playing => "Playing",
            ServerState::Stopped => "Stopped",
            ServerState::Interactive => "Interactive",
        }
    }
}
