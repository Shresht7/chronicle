use serde::Serialize;

#[derive(Serialize)]
pub struct ApiSnapshot {
    pub id: i64,
    pub timestamp: String,
    pub files_count: i64,
    pub total_size: i64,
}

#[derive(Debug, Serialize)]
pub struct ApiNode {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ApiNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}
