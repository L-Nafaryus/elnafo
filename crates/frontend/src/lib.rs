use askama_axum::Template;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist/assets/"]
pub struct Assets;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate;

#[test]
fn test_render() {
    println!("{}", HomeTemplate.render().unwrap());
}
