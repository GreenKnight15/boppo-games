# Game ideas

Backlog of mini-game ideas for Boppo (10 light-up buttons + audio, no screen).

## Soundboards (easiest — same code shape as fart-machine)

- **Drum kit / beat pad** — kick, snare, hats, toms, cymbals across the buttons; loop mode later
- **Animal barn** — each button an animal sound + themed color (cow = white, pig = pink)
- **Piano** — 10 notes on a pentatonic scale so mashing always sounds pleasant
- **Fart machine** — done: `games/fart-machine`

## Simple game loops (one mechanic + `update_loop`)

- **Whack-a-mole** — a button lights up, press before it goes dark; speeds up each round
- **Simon** — light+sound sequence grows each round; kid repeats it
- **Reaction duel** — all lights red, wait for green, first press wins; false start = fart sound
- **Lights out** — pressing a button toggles itself + neighbors; goal is all dark; pure logic, no audio assets

## Ambitious

- **Hot potato** — lit "potato" bounces between buttons; pass the device around; explosion sound = you're out
- **Freeze dance** — music + light chase randomly stops; pressing while frozen = you're out
- **Beat loop station** — presses quantized to a beat and looped; build a track layer by layer
