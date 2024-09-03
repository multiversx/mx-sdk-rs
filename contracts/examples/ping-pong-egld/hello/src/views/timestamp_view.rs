use rocket::response::content::RawText;

pub fn timestamp_view(response: String) -> RawText<String> {
    RawText(response) // Directly return the string as plain text
}
