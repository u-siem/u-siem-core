pub mod tasks;
pub mod commands;
pub struct Message<M> {
    pub msg_id : u64,
    pub msg : M
}