use std::collections::HashMap;


pub struct Data {
    pub level: Level,
} // User data, which is stored and accessible in all command invocations


pub struct Level {
    pub users: HashMap<u64, u64>
}
