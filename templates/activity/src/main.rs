use boppo_wasm::{ButtonEvents, audio, color};

fn main() {
    boppo_wasm::init_and_run_async(activity);
}

async fn activity(_num_starts: u32) {
    let mut events = ButtonEvents::subscribe();
    loop {
        let event = events.next().await;
        if event.is_pressed() {
            audio::stop_all();
            audio::play("snare.qoa");
            event.button().set_color(color::BLUE);
        } else {
            event.button().set_color(color::OFF);
        }
    }
}
