# ternary-jam

**A jam session in code. Agents that improvise, listen, respond, and surprise you.**

A jam session isn't rehearsed. It's not scripted. Someone plays something, someone else responds, the energy builds, the room finds a groove, and for a few minutes everyone is thinking the same thought at the same time. Then it dissolves, reforms, goes somewhere new.

This crate models that. Multiple ternary agents sit in a room. Each one has an instrument (a strategy for generating ternary values). They can hear each other. They respond to what they hear. Sometimes they lead, sometimes they follow. Sometimes they play something unexpected and everyone adjusts.

The `JamSession` is the room. The `Player` is each musician. The `energy` is the vibe — high energy means everyone's playing hard, low energy means they're listening. The `calls` and `responses` are the musical conversation.

## What's Inside

- **`JamSession`** — the room. Manages players, rounds, energy tracking, and history
- **`Player`** — a musician with a strategy, energy level, and response tendency
- **`jam_round(session)`** — one round of the jam. Each player responds to the previous round
- **`call_and_response(caller, responder, history)`** — the fundamental musical conversation. One player calls, the other responds
- **`energy_level(session)`** — measure the room's energy. High = lots of ±1 activity, low = lots of 0
- **`groove_detector(history)`** — has the room found a repeating pattern? Is there a groove?
- **`surprise_score(play, expected)`** — how unexpected was this play? Surprise drives musical interest
- **`trade_fours(players, rounds)`** — jazz tradition: players take 4-bar solos in turn

## Quick Example

```rust
use ternary_jam::*;

// Set up a jam with 4 players
let mut session = JamSession::new(4);

// Player 0 leads with a pattern
session.set_lead(0, vec![1, 0, -1, 0, 1, 0, -1, 0]);

// Run 16 rounds of improvisation
for _ in 0..16 {
    jam_round(&mut session);
}

// Check the energy
let energy = energy_level(&session);
println!("Room energy: {:.2}", energy);
// High energy = everyone's playing. Low = they're listening.

// Detect if a groove emerged
if let Some(pattern) = groove_detector(&session.history) {
    println!("Found a groove! Pattern: {:?}", pattern);
}

// Trade fours: players take turns soloing
let solos = trade_fours(&session.players, 4);
// Player 0 solos for 4 bars, then player 1, then player 2, etc.
```

## The Deeper Truth

**Jamming is the hardest thing to model in music AI.** It requires listening (processing other players' output), taste (knowing what to play in response), restraint (not playing too much), and surprise (doing the unexpected at the right moment). In ternary, all of this reduces to a sequence of {-1, 0, +1} values — but the *meaning* of each value depends on context. A +1 after a long stretch of 0s is a shout. The same +1 in a wall of +1s is just noise.

The energy level captures this: it's not just "how many non-zero values" but "how much has the activity *changed* from the previous round." A sudden burst of activity after calm = energy spike. Gradual building = energy rise. The energy curve IS the emotional arc of the jam.

The groove detector looks for periodicity in the collective output. When all players lock into a repeating pattern — even a simple one like [1, 0, -1, 0] — that's a groove. It's the ternary equivalent of a rhythm section locking in. The groove can be stable (everyone keeps it) or unstable (it dissolves after a few rounds). The most interesting jams have unstable grooves that form, dissolve, and reform in new shapes.

**Use cases:**
- **Generative music** — create evolving, responsive compositions
- **Game audio** — background music that responds to gameplay energy
- **AI music research** — model improvisation and musical conversation
- **Education** — teach improvisation concepts through simulation
- **Live performance** — let the code jam with human musicians

## See Also

- **ternary-rhythm** — rhythm patterns that players use
- **ternary-phase** — phase alignment between players in a jam
- **ternary-polyrhythm** — multiple players with different pulse rates
- **ternary-sync** — Z₃ synchronization (when the jam locks in)
- **ternary-muse** — creative inspiration for generating musical ideas
- **ternary-ear** — ear training (teaching agents to listen better)
- **ternary-kuramoto** — what happens when jamming fails to synchronize

## Install

```bash
cargo add ternary-jam
```

## License

MIT
