#[macro_use]
extern crate rocket;
mod paste_id;
mod paste_form;

use std::fs::File;
use rocket::response::Debug;
use rocket::data::ToByteUnit;
use rocket::Data;

use paste_id::PasteId;

const ID_SIZE: usize = 3;
const HOST: &str = "http://localhost:8000";

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<String, Debug<std::io::Error>> {
    let id = PasteId::new(ID_SIZE);
    let filepath = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);
    
    paste.open(128.kibibytes()).into_file(filepath).await?;

    Ok(url)
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    let filepath = format!("upload/{id}", id = id);
    File::open(&filepath).ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload, retrieve])
}
