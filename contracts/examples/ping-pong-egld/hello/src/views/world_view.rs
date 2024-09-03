use rocket::response::content::RawText;

pub fn world_view() -> RawText<&'static str> {
    RawText("Hello, world!")
}
