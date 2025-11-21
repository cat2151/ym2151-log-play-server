/// Server state representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerState {
    Playing,
    Stopped,
    Interactive,
}
