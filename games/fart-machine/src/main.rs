use boppo_wasm::color::rgb::Rgb;
use boppo_wasm::{Button, ButtonEvents, audio, color};

/// Sound file and color for each button, indexed by `Button::index()`.
/// Files live in `assets/`; `megafart.mp3` is optional (all-buttons combo).
const FARTS: [(&str, Rgb<u8>); Button::COUNT] = [
    ("apebble-fart-1-228241.mp3", color::GREEN),
    ("apebble-fart-2-228242.mp3", color::CHARTREUSE),
    ("apebble-fart-3-228243.mp3", color::YELLOW),
    ("apebble-fart-4-228244.mp3", color::ORANGE),
    ("apebble-fart-5-228245.mp3", color::RED),
    ("apebble-fart-6-228246.mp3", color::PINK),
    ("apebble-fart-7-228247.mp3", color::MAGENTA),
    ("apebble-fart-8-228248.mp3", color::PURPLE),
    ("apebble-fart-9-228249.mp3", color::BLUE),
    ("freesound_community-fartie-107647.mp3", color::CYAN),
];

const MEGA_FART: &str = "megafart.mp3";
const IDLE_BRIGHTNESS: f32 = 0.3;

fn main() {
    boppo_wasm::init_and_run_async(activity);
}

async fn activity(_num_starts: u32) {
    for i in 0..Button::COUNT {
        Button::from_index(i).set_color(idle_color(i));
    }

    let mut pressed = [false; Button::COUNT];
    let mut events = ButtonEvents::subscribe();
    loop {
        let event = events.next().await;
        let button = event.button();
        let i = button.index();
        pressed[i] = event.is_pressed();
        if event.is_pressed() {
            audio::stop_all();
            let _ = audio::try_play(FARTS[i].0);
            button.set_color(FARTS[i].1);
            if pressed.iter().all(|&p| p) {
                let _ = audio::try_play(MEGA_FART);
            }
        } else {
            button.set_color(idle_color(i));
        }
    }
}

fn idle_color(i: usize) -> Rgb<u8> {
    color::dim_to(FARTS[i].1, IDLE_BRIGHTNESS)
}
