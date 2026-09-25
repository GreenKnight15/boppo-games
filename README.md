# boppo-games

Mini-games for the [Boppo](https://boppo.com) tablet — a screen-free device with
10 light-up buttons, a speaker, and Wi-Fi — written in Rust and compiled to
WebAssembly. Each game is a Boppo **activity** built with the
[`boppo_wasm`](https://docs.rs/boppo_wasm) crate and deployed with the
[`boppo` CLI](https://github.com/boppofun/boppo_cli).

## Repository layout

```
boppo-games/
├── .cargo/config.toml      # builds target wasm32-wasip1 repo-wide
├── rust-toolchain.toml     # stable Rust + wasm32-wasip1 target
├── justfile                # optional convenience recipes
├── games/                  # one directory per mini-game
│   ├── IDEAS.md            #   brainstormed game backlog
│   └── fart-machine/       #   10 buttons, 10 farts
└── templates/
    └── activity/           # starter template — copy this to create a new game
```

Each game is a **standalone crate** rather than a Cargo workspace member:
`boppo activity build/deploy` expects the compiled `.wasm` under the project's
own `target/` directory.

## Prerequisites

* [Rust](https://rustup.rs) — `rust-toolchain.toml` auto-selects stable and the
  `wasm32-wasip1` target
* [`boppo` CLI](https://github.com/boppofun/boppo_cli) — `cargo install boppo_cli`
* [`wasm-opt`](https://github.com/WebAssembly/binaryen/releases) *(optional)* —
  required unless you build with `--no-optimize`
* [`just`](https://github.com/casey/just) *(optional)* — shortcut recipes

## Creating a new game

```bash
just new-game my-game
```

Or manually:

```bash
cp -r templates/activity games/my-game
# then set the package name in games/my-game/Cargo.toml
```

> **Crate names must use underscores, not hyphens** — `cargo build` emits
> `my-game.wasm` but `boppo activity deploy` looks for `my_game.wasm`.
> `just new-game` converts for you; set `name = "my_game"` by hand otherwise.

The package name becomes the activity directory on the device
(`/sd/activities/user/wasm/<name>/`).

## Game structure

```
games/my-game/
├── Cargo.toml        # depends on boppo_wasm
├── src/main.rs       # activity entry point
├── assets/           # files deployed alongside the wasm (mp3/qoa audio, etc.)
└── target/           # local build output (git-ignored)
```

## Building and deploying

Pair the device first (approves on the device itself):

```bash
boppo wifi discover
```

Then:

```bash
cd games/my-game
boppo activity deploy             # build, sync, and start over Wi-Fi
boppo activity start              # start the already-deployed activity
```

Or from the repo root: `just deploy my-game`, `just start my-game`.
Without `wasm-opt`, add `--no-optimize`.

Deploy syncs the `.wasm` + everything in `assets/` to
`/sd/activities/user/wasm/<name>/` and runs
`start wasm user/wasm/<name>/<name>.wasm`.

## Adding the game to a device menu

Requires Developer Mode (Settings → Device Info → hold the outer 2 buttons on
each side for 5 seconds). Device menus are plain JSON on the SD card:

```
/sd/activities/menu/<category>.menu.json   # button → shortcut + color map
/sd/activities/shortcuts/<name>            # one-line launch command
/sd/activities/shortcuts/<name>.si.json    # sound played as the button label
```

To add a button for a custom activity:

```bash
# 1. Shortcut — the command that launches the activity
boppo wifi execute-command "create_file '/sd/activities/shortcuts/fart_machine' 'start wasm user/wasm/fart_machine/fart_machine.wasm'"

# 2. Button label — a Sound Instruction played when the button is focused
#    (paths are relative to /sd/activities/)
boppo wifi execute-command 'create_file '"'"'/sd/activities/shortcuts/fart_machine.si.json'"'"' '"'"'["user/wasm/fart_machine/my_label_sound.mp3"]'"'"''

# 3. Add an entry to a category menu — entries map to buttons in order.
#    Rewrite the menu JSON with your entry appended (single-line JSON is fine):
boppo wifi execute-command 'create_file '"'"'/sd/activities/menu/number_activities.menu.json'"'"' '"'"'{"instructions":{...},"entries":[...,{"shortcut":"fart_machine","color":"YELLOW"}]}'"'"''
```

Then launch the menu to test:

```bash
boppo wifi execute-command "start menu menu/number_activities.menu.json"
```

Stock menus: `arcade`, `audio_books`, `color_activities`, `favorites`,
`number_activities`, `podcasts`, `sound_activities`, `young_explorer`.
Firmware updates may restore stock menu files, so edits may need re-applying.

## Audio assets

* `audio::play` accepts **mp3** and **qoa** directly — no conversion needed
* Per the [Activity Guidelines](https://developer.boppo.com/docs/activity-guidelines):
  ~10 simultaneous sounds max, call `audio::stop_all()` before per-press
  playback, trim leading dead-space in clips, keep at least one light on
* Filenames in code must match `assets/` exactly

## Attribution

Fart machine sounds are from [Pixabay](https://pixabay.com), whose
[Content License](https://pixabay.com/service/license-summary/) allows free use
without attribution. If any file came from freesound.org instead, check its
license — many there require attribution (CC-BY).

The repo's LICENSE covers the code only; audio assets remain under their
original licenses.

## Reference

* [`boppo_wasm` API docs](https://docs.rs/boppo_wasm)
* [`boppo_core` API docs](https://docs.rs/boppo_core)
* [Boppo developer docs](https://developer.boppo.com)
* [Official activity template](https://github.com/boppofun/rust_boppo_wasm_template)
* [Commands reference](https://developer.boppo.com/docs/commands)
