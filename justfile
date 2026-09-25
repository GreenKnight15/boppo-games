# Convenience recipes — `just` is optional; all commands work without it.

# Create a new game by copying the starter template
new-game NAME:
    cp -r templates/activity games/{{NAME}}
    sed -i 's/^name = ".*"/name = "'"$(echo {{NAME}} | tr '-' '_')"'"/' games/{{NAME}}/Cargo.toml

# Compile a game to wasm32-wasip1 (runs wasm-opt if installed)
build NAME:
    cd games/{{NAME}} && boppo activity build

# Build, sync, and start a game on a paired Boppo over Wi-Fi
deploy NAME:
    cd games/{{NAME}} && boppo activity deploy

# Start the already-deployed game on the device
start NAME:
    cd games/{{NAME}} && boppo activity start
