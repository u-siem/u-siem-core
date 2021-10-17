use serde::Serialize;


/// Enqueued task with data.
/// If the Task has finished then the result has Some data.
/// This data can be a Ok with the output (not the data) or the error.
/// The ID is to get the Task result
#[derive(Serialize, Debug, Clone)]
pub struct SiemTask {
    pub created_at : i64,
    pub enqueued_at : i64,
    pub origin : String,
    pub id : u64,
    pub data : serde_json::Value
}

#[derive(Serialize, Debug, Clone)]
pub struct SiemTaskResult {
    pub id : u64,
    pub data : Option<serde_json::Value>
}