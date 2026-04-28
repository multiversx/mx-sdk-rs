extern crate very_large_storage_interactor;

#[tokio::main]
pub async fn main() {
    very_large_storage_interactor::very_large_storage_cli().await;
}
