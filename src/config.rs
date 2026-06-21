use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("FlappyBird.rs"),
        window_width: 432, // kenapa 432? karena width background itu 288px, dikali 1.5 = 432. =)
        window_height: 768, // base: 512
        window_resizable: false,
        // sample_count: 1,
        ..Default::default()
    }
}