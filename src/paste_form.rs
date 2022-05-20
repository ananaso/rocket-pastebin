use rocket::FromForm;

#[derive(FromForm)]
pub struct PasteForm<'r> {
    pub content: &'r str,
}
