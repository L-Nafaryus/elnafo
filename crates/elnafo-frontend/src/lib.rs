use askama_axum::Template;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist/resources/assets/"]
pub struct Assets;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub view: &'a str,
}

#[test]
fn test_render() {
    println!("{}", BaseTemplate { view: "home" }.render().unwrap());
}
