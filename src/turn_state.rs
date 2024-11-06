#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn
}