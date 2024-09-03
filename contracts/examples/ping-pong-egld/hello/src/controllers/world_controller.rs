use rocket::{get, response::content::RawText};
use crate::views::world_view::world_view;

#[get("/world")]
pub fn world() -> RawText<&'static str> {
    world_view()
}
