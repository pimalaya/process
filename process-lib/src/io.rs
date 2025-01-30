#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    SpawnThenWait,
    SpawnThenWaitWithOutput,
}
