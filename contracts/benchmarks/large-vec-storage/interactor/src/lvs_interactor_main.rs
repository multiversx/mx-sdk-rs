extern crate large_vec_storage_interactor;

#[tokio::main]
pub async fn main() {
    large_vec_storage_interactor::large_vec_storage_cli().await;
}
