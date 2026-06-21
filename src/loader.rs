use macroquad::prelude::*;
use std::error::Error;

// #[derive(Copy)]
pub struct FImage { // kalo ditanya kenapa namanya FImage, you know... FlappyBird Image... 
    pub tex: Texture2D,
    pub params: DrawTextureParams,
}

pub struct Bird {
    pub tex_down: Texture2D,
    pub tex_mid: Texture2D,
    pub tex_up: Texture2D,
    pub params: DrawTextureParams,
}

pub fn load_image(file: &[u8]) -> Result<FImage, Box<dyn Error>> {
    let img = Image::from_file_with_format(file, None)?;
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    let params = DrawTextureParams {
        dest_size: Some(vec2(tex.width() * 1.5, tex.height() * 1.5)),
        ..Default::default()
    };

    Ok(FImage {tex, params})
}

fn load_bird_separately(file: &[u8]) -> Result<Texture2D, Box<dyn Error>> {
    let img = Image::from_file_with_format(file, None)?;
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);

    Ok(tex)
}

pub fn load_bird() -> Result<Bird, Box<dyn Error>> {
    let tex_down = load_bird_separately(include_bytes!("../assets/sprites/yellowbird-downflap.png"))?; 
    let tex_mid = load_bird_separately(include_bytes!("../assets/sprites/yellowbird-midflap.png"))?; 
    let tex_up = load_bird_separately(include_bytes!("../assets/sprites/yellowbird-upflap.png"))?; 
    let params = DrawTextureParams {
        dest_size: Some(vec2(tex_up.width() * 1.5, tex_up.height() * 1.5)),
        ..Default::default()
    };

    Ok(Bird {tex_down, tex_mid, tex_up, params})
}