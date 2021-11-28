#[derive(Debug)]
pub enum Action {
    Build,
    TestInputOutput,
}

#[derive(Debug)]
pub struct Actions(pub Vec<Action>);
