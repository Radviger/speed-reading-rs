extern crate core;

use std::fs::File;
use std::io::Read;
use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    dragging: usize,
    asset: Option<Asset<String>>,
    words: Vec<String>,
    speed: f32,
    current_word: f32,
}

// Create a new asset loaded to load .txt files as strings
fn create_text_loader() -> AssetLoader {
    AssetLoader::new().use_parser(parse_text).extension("txt")
}

// This parses the &[u8] from the file to the type that we want, string in this case
fn parse_text(_id: &str, data: Vec<u8>) -> Result<String, String> {
    String::from_utf8(data).map_err(|e| e.to_string())
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_loader(create_text_loader())
        .add_config(DrawConfig)
        .add_config(notan::log::LogConfig::new(notan::log::LevelFilter::Debug))
        .draw(draw)
        .event(event)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/ubuntu.ttf"))
        .unwrap();
    State {
        font,
        dragging: 0,
        words: Vec::new(),
        speed: 60.0,
        current_word: 0.0,
        asset: None
    }
}

fn event(assets: &mut Assets, state: &mut State, evt: Event) {
    match evt {
        Event::DragEnter { .. } => {
            state.dragging += 1;
        }
        Event::DragLeft => {
            state.dragging = 0;
        }
        Event::MouseWheel { delta_x, delta_y } => {
            state.speed += 10.0 * delta_y.signum();
            if state.speed <= 10.0 {
                state.speed = 10.0;
            }
        }
        Event::Drop(file) => {
            state.dragging = 0;

            if file.mime == "text/plain" {
                state.asset = Some(assets.load_dropped_file::<String>(&file).unwrap());
            }
        }
        _ => {}
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let frame_time = app.timer.delta_f32();

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    if state.words.is_empty() {
        if let Some(asset) = &state.asset {
            if asset.is_loaded() {
                let text = state.asset.take().unwrap().try_unwrap().unwrap();
                for part in text.split([' ', '\n']) {
                    state.words.push(part.to_owned());
                }
            }
        }
    }

    // Just UI Text
    if state.dragging == 0 {
        if state.words.is_empty() {
            let text = "Перетащите сюда текстовый файл";
            draw.text(&state.font, text)
                .color(Color::ORANGE)
                .size(30.0)
                .v_align_middle()
                .h_align_center()
                .position(400.0, 300.0);
        } else {
            let index = (state.current_word as usize).min(state.words.len() - 1);
            let word = &state.words[index];
            draw.text(&state.font, word)
                .color(Color::WHITE)
                .size(30.0)
                .v_align_middle()
                .h_align_center()
                .position(400.0, 300.0);

            draw.text(&state.font, &format!("{} слов в минуту", state.speed))
                .color(Color::ORANGE)
                .size(24.0)
                .v_align_middle()
                .position(25.0, 25.0);

            state.current_word += state.speed / 60.0 * frame_time;
        }
    } else {
        draw.rect((10.0, 10.0), (780.0, 580.0))
            .color(Color::WHITE)
            .stroke(6.0);

        let text = format!("Вы перетаскиваете {} файл", state.dragging);
        draw.text(&state.font, &text)
            .size(30.0)
            .color(Color::GRAY)
            .v_align_middle()
            .h_align_center()
            .position(400.0, 300.0);
    }

    gfx.render(&draw);
}