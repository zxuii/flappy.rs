use macroquad::{audio::*, prelude::*};
// use macroquad::rand::gen_range;
use ::rand::RngExt;
use std::error::Error;
use std::collections::VecDeque;


mod config;
mod loader;
use config::window_conf;

pub type R = Result<(), Box<dyn Error>>;

enum BirdAnimation {
    Up,
    Mid,
    Down,
}

fn gen_rand_pipe(pipe: &loader::FImage) -> (Rect, Rect, Rect, bool) {
    let mut rng = ::rand::rng();
    let random = rng.random_range(-150..=150);
    let center_pos = vec2(center(screen_width()), center(screen_height()));
    let pipe_rect = Rect {
        w: pipe.tex.width() * 1.5,
        h: pipe.tex.height() * 1.5,
        x: screen_width(),
        y: center_pos.y + random as f32,
    };
    let score_collision_rect = Rect {
        w: pipe.tex.width(),
        h: pipe.tex.height() * 1.5,
        x: screen_width() + 25.0,
        y: center_pos.y + random as f32 - 325.0,
    };
    let pipe_flipped_rect = Rect {
        w: pipe.tex.width() * 1.5,
        h: pipe.tex.height() * 1.5,
        x: screen_width(),
        y: center_pos.y - 650.0 + random as f32,
    };
    (pipe_rect, pipe_flipped_rect, score_collision_rect, false)
}

#[macroquad::main(window_conf())]
async fn main() -> R {
    let bird = loader::load_bird()?;
    let mut bird_animation_timer = 0.0;
    let bird_animation_interval = 0.1;
    let pipe_spawn_interval = 1.7;
    let mut pipe_spawn_timer = 0.0;

    let gravity = 1000.0;
    let obs_velocity = 140.0; // constant untuk kecepatan obstacle 
    let jump_force = -350.0;

    let mut bird_animation = BirdAnimation::Down;
    let bg = loader::load_image(include_bytes!("../assets/sprites/background-day.png"))?;
    let ground = loader::load_image(include_bytes!("../assets/sprites/base.png"))?;
    let press_start_msg = loader::load_image(include_bytes!("../assets/sprites/message.png"))?;
    let game_over_msg = loader::load_image(include_bytes!("../assets/sprites/gameover.png"))?;
    let mut score = 0;
    let center_pos: Vec2 = vec2(center(screen_width()), center(screen_height()));
    let text_font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/FlappyBirdRegular.ttf"))?;
    let pipe = loader::load_image(include_bytes!("../assets/sprites/pipe-green.png"))?;
    let mut pipe_flipped = loader::load_image(include_bytes!("../assets/sprites/pipe-green.png"))?;
    let mut is_dead_sfx_played = false;
    let mut is_hit_sfx_played = false;

    pipe_flipped.params.flip_y = true;

    let mut pipes: VecDeque<(Rect, Rect, Rect, bool)> = VecDeque::new();
    let random_pipe = gen_rand_pipe(&pipe);
    pipes.push_back(random_pipe);

    let (bird_x, mut bird_y) = (
        center_pos.x - ((bird.tex_up.width() / 2.0) * 1.5),
        center_pos.y - ((bird.tex_up.height() / 2.0) * 1.5),
    );

    let ground_y = screen_height() - ground.tex.height() * 1.5;
    let mut ground_x: [f32; 2] = [0.0, ground.tex.width() * 1.5];

    let mut bird_y_velocity = 0.0;
    let mut is_dead = false;
    let mut is_start = false;

    let wing_sfx = load_sound_from_bytes(include_bytes!("../assets/audio/wing.wav")).await?;
    let die_sfx = load_sound_from_bytes(include_bytes!("../assets/audio/die.wav")).await?;
    let point_sfx = load_sound_from_bytes(include_bytes!("../assets/audio/point.wav")).await?;
    let hit_sfx = load_sound_from_bytes(include_bytes!("../assets/audio/hit.wav")).await?;

    loop {
        let bird_rect = Rect {
            w: bird.tex_up.width() * 1.5,
            h: bird.tex_up.height() * 1.5,
            x: bird_x,
            y: bird_y,
        };

        let ground_rect = Rect {
            w: ground.tex.width() * 1.5,
            h: ground.tex.height() * 1.5,
            x: 0.0,
            y: ground_y,
        };
        let dt = get_frame_time();

        clear_background(BLACK);
        draw_texture_ex(&bg.tex, 0.0, 0.0, WHITE, bg.params.clone());
        if !is_start {
            draw_texture_ex(
                &press_start_msg.tex,
                center_pos.x - ((press_start_msg.tex.width() / 2.0) * 1.5),
                center_pos.y - ((press_start_msg.tex.height() / 2.0) * 1.5) - 70.0,
                WHITE,
                press_start_msg.params.clone(),
            );
            // loncat pertama
            if is_mouse_button_pressed(MouseButton::Left) {
                is_start = true;
                play_sound_once(&wing_sfx);

                bird_y_velocity = jump_force;
                // bird.params.rotation = -0.35;
            }
            draw_texture_ex(
                &ground.tex,
                ground_x[0],
                ground_y,
                WHITE,
                ground.params.clone(),
            );

            // bird tanpa animasi
            // draw_texture_ex(&bird.tex_down, bird_x, bird_y, WHITE, bird.params.clone());
        } else {
            // kalo mati maka jatuh selamanya
            if !is_dead {
                // logic lompat
                if is_mouse_button_pressed(MouseButton::Left) {
                    // kurangi velocity yang berakibat menambah tinggi dari `bird` dengan cara diapply di bagian `apply velocity` agar
                    // tak langsung begitu saja berubah dengan statis. kalau ditanya kenapa?
                    // karena kita harus membuat physics secara manual.
                    play_sound_once(&wing_sfx);
                    bird_y_velocity = jump_force;
                    // bird.params.rotation = -0.35;
                }

                // cek kena tanah atau kaga
                if bird_rect.overlaps(&ground_rect) {
                    if !is_hit_sfx_played {
                        play_sound_once(&hit_sfx);
                        is_hit_sfx_played = true;
                    }
                    is_dead = true;
                }
            }
            // if bird.params.rotation <= 1.0 {
            //     bird.params.rotation += 0.01 * dt;
            // }
            // jatuh
            bird_y_velocity += gravity * dt;

            // apply velocity
            bird_y += bird_y_velocity * dt;

            // cek ground ke 2 sudah ke titik pojok kiri layar atau belum
            // kalau ditanya kenapa? karena begini:
            // [posisi_ground_x_1, posisi_ground_x_2]
            //                     ^^^^^^^^^^^^^^^^^-- ini adalah ground ke 2
            // ground ke 2 jika berada di bagian kiri layar dia akan otomatis
            // mengubah posisi kedua ground menjadi ke titik awal.
            // menyebabkan ilusi mata yang terlihat bergerak tak terbatas.
            // kenapa enggak ground pertama?
            // karena ground pertama posisinya memang sudah di sudut kiri,
            // jadi tak memungkinkan hal itu terjadi dan hanya menyebabkan dirinya
            // diam tak bergerak.
            if !is_dead {
                if ground_x[1] >= 0.0 {
                    ground_x[0] -= obs_velocity * dt; // pergerakan ground
                    ground_x[1] -= obs_velocity * dt; // 
                } else {
                    // reset posisi kedua ground
                    ground_x = [0.0, ground.tex.width() * 1.5];
                }
            }

            if !is_dead {
                bird_animation_timer += dt;
            }
            if bird_animation_timer >= bird_animation_interval {
                bird_animation_timer -= bird_animation_interval;
                match bird_animation {
                    BirdAnimation::Down => {
                        bird_animation = BirdAnimation::Mid;
                    }
                    BirdAnimation::Mid => {
                        bird_animation = BirdAnimation::Up;
                    }
                    BirdAnimation::Up => {
                        bird_animation = BirdAnimation::Down;
                    }
                }
            }

            // pipe_rect = rect umm salah taruh

            // ternyata simple untuk ketinggian random tiap pipe nya.
            // MAX 150
            // MIN -150
            // dengan catatan untuk flipped pipe (pipe bagian atas)
            // di adjust (-650.0) pada bagian y nya dari titik tengah layar.

            // Pipes spawning per-interval
            if !is_dead {
                pipe_spawn_timer += dt;

                if pipe_spawn_timer >= pipe_spawn_interval {
                    pipe_spawn_timer -= pipe_spawn_interval;
                    let random_pipe = gen_rand_pipe(&pipe);
                    pipes.push_back(random_pipe);
                }

                if pipes.len() >= 5 {
                    pipes.pop_front();
                }
            }
            for i in &mut pipes {
                if !is_dead {
                    i.0.x -= obs_velocity * dt;
                    i.1.x -= obs_velocity * dt;
                    i.2.x -= obs_velocity * dt;
                }
                draw_texture_ex(&pipe.tex, i.0.x, i.0.y, WHITE, pipe.params.clone());
                draw_texture_ex(&pipe.tex, i.1.x, i.1.y, WHITE, pipe_flipped.params.clone());
                // draw_rectangle(i.2.x, i.2.y, i.2.w, i.2.h, BLACK);

                if bird_rect.overlaps(&i.0) {
                    if !is_hit_sfx_played {
                        play_sound_once(&hit_sfx);
                        is_hit_sfx_played = true;
                    }
                    is_dead = true;
                }
                if bird_rect.overlaps(&i.1) {
                    if !is_hit_sfx_played {
                        play_sound_once(&hit_sfx);
                        is_hit_sfx_played = true;
                    }
                    is_dead = true;
                }
                if bird_rect.overlaps(&i.2) {
                    score += if i.3 {
                        0
                    } else {
                        play_sound_once(&point_sfx);
                        1
                    };
                    i.3 = true;
                }
            }

            // ground
            draw_texture_ex(
                &ground.tex,
                ground_x[0],
                ground_y,
                WHITE,
                ground.params.clone(),
            );
            draw_texture_ex(
                &ground.tex,
                ground_x[1],
                ground_y,
                WHITE,
                ground.params.clone(),
            );

            match bird_animation {
                BirdAnimation::Down => {
                    draw_texture_ex(&bird.tex_down, bird_x, bird_y, WHITE, bird.params.clone());
                }
                BirdAnimation::Mid => {
                    draw_texture_ex(&bird.tex_mid, bird_x, bird_y, WHITE, bird.params.clone());
                }
                BirdAnimation::Up => {
                    draw_texture_ex(&bird.tex_up, bird_x, bird_y, WHITE, bird.params.clone());
                }
            }
            let msg = score.to_string();
            // println!("{}", score);
            let font_size = 100;
            let dimensions = measure_text(&msg, Some(&text_font), font_size, 1.0);
            draw_text_ex(
                msg,
                center_pos.x - (dimensions.width / 2.0),
                center_pos.y + (dimensions.height / 2.0) - dimensions.offset_y - 200.0,
                TextParams {
                    font_size,
                    font: Some(&text_font),
                    ..Default::default()
                },
            );
        }

        // for (i, k) in ground_x.iter().enumerate() {
        //     println!("{}, {}", i, k);
        // }

        // biar ga ngechit terbang tinggi keluar window
        if bird_y < -40.0 {
            if !is_hit_sfx_played {
                play_sound_once(&hit_sfx);
                is_hit_sfx_played = true;
            }
            is_dead = true;
        }
        if is_dead {
            if !is_dead_sfx_played {
                play_sound_once(&die_sfx);
                is_dead_sfx_played = true;
            }
            draw_texture_ex(
                &game_over_msg.tex,
                center_pos.x - ((game_over_msg.tex.width() / 2.0) * 1.5),
                center_pos.y - ((game_over_msg.tex.height() / 2.0) * 1.5) - 70.0,
                WHITE,
                game_over_msg.params.clone(),
            );

            // yang penting work dulu, kode bloat biarin awokawokawok
            let msg = "PRESS 'R'";
            let font_size = 50;
            let dimensions = measure_text(msg, Some(&text_font), font_size, 1.0);
            draw_text_ex(
                msg,
                center_pos.x - (dimensions.width / 2.0),
                center_pos.y + (dimensions.height / 2.0) - dimensions.offset_y + 20.0,
                TextParams {
                    font_size,
                    font: Some(&text_font),
                    ..Default::default()
                },
            );

            ////////////////////////////////////////////////

            let msg = "TO RESTART THE GAME";
            let font_size = 50;
            let dimensions = measure_text(msg, Some(&text_font), font_size, 1.0);
            draw_text_ex(
                msg,
                center_pos.x - (dimensions.width / 2.0),
                center_pos.y + (dimensions.height / 2.0) - dimensions.offset_y + 50.0,
                TextParams {
                    font_size,
                    font: Some(&text_font),
                    ..Default::default()
                },
            );

            // resett.....
            // uggh manual karena dari awal udah ga pake `GameState` thinky jadinya...
            // ya begini awokawokawok
            // demi kaga menghabiskan waktu lebih banyak lagi mending begini ae
            if is_key_pressed(KeyCode::R) {
                is_dead = false;
                is_start = false;
                bird_y = center_pos.y - ((bird.tex_up.height() / 2.0) * 1.5);
                ground_x = [0.0, ground.tex.width() * 1.5];
                pipe_spawn_timer = 0.0;
                pipes.clear();
                score = 0;
                is_dead_sfx_played = false;
                is_hit_sfx_played = false;
                // toh beberapa udah dihandle secara ga sengaja owawokawo
            }
        }

        next_frame().await
    }
}

// jujur, ini ga guna samsek buset
fn center(x: f32) -> f32 {
    x / 2.0
}
