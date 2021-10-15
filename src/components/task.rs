use serde::Serialize;


/// Enqueued task with data.
/// If the Task has finished then the result has Some data.
/// This data can be a Ok with the output (not the data) or the error.
/// The ID is to get the Task result
#[derive(Serialize, Debug)]
pub struct SiemTask {
    pub created_at : u64,
    pub enqueued_at : u64,
    pub origin : String,
    pub id : String,
    pub data : serde_json::Value,
    pub description : String,
    pub result : Option<(u64, Result<String, String>)>
}
#[derive(Serialize, Debug)]
pub struct SiemTaskResult {
    pub id : String,
    pub data : Option<serde_json::Value>
}