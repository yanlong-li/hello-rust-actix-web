use std::sync::Mutex;

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>,
}

mod user;