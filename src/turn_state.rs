/// Перечисление игровых состояний.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameStart,
    GameOver,
    NextLevel,
    Victory,
    Exit
}