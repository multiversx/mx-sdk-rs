#[derive(Debug, Clone)]
pub struct Log {
    pub address: String,
    pub endpoint: String,
    pub topics: Vec<String>,
    pub data: String,
}
