#[derive(Debug, Serialize)]
pub struct ServicePost {
    id: String,
    user_id: String,
    body: String,
    // created_data: Timestamp
}