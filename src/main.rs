#[macro_use]
extern crate rocket;
mod paste_form;
mod paste_id;

use rocket::data::ToByteUnit;
use rocket::form::{Form, Strict};
use rocket::fs::{relative, FileServer};
use rocket::response::Debug;
use rocket::Data;
use std::fs::{remove_file, File};
use std::io::Write;

use paste_form::PasteForm;
use paste_id::PasteId;

const ID_SIZE: usize = 3;
const HOST: &str = "http://localhost:8000";
const PASTE_LOCATION: &str = "upload";

#[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> Result<(), Debug<std::io::Error>> {
    let filepath = format!("{folder}/{id}", folder = PASTE_LOCATION, id = id);
    match remove_file(filepath) {
        Ok(_) => Ok(()),
        Err(error) => Err(Debug(error)),
    }
}

#[post("/", data = "<paste>")]
async fn paste(paste: Form<Strict<PasteForm<'_>>>) -> Result<String, Debug<std::io::Error>> {
    let id = PasteId::new(ID_SIZE);
    let filepath = format!("{folder}/{id}", folder = PASTE_LOCATION, id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);

    let mut file = File::create(filepath)?;

    match write!(file, "{}", paste.into_inner().content) {
        Ok(_) => Ok(url),
        Err(error) => Err(Debug(error)),
    }
}

#[post("/", data = "<paste>", rank = 2)]
async fn upload(paste: Data<'_>) -> Result<String, Debug<std::io::Error>> {
    let id = PasteId::new(ID_SIZE);
    let filepath = format!("{folder}/{id}", folder = PASTE_LOCATION, id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);

    paste.open(128.kibibytes()).into_file(filepath).await?;

    Ok(url)
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    let filepath = format!("{folder}/{id}", folder = PASTE_LOCATION, id = id);
    File::open(&filepath).ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![upload, retrieve, paste, delete])
        .mount("/", FileServer::from(relative!("html")))
}
