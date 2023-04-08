use crate::response::image;

#[get("/image/<name>")]
pub fn png(name: String) -> image::Png {
    image::Png::new(&("/image/".to_string() + name.as_str()))
}
