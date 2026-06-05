# ternary-jam: Musical jam session as multi-agent coordination

Musical jam session where agents improvise, compete for harmonic space, and cooperate to create consonance — all within a ternary ({-1, 0, +1}) harmonic framework. The jam session IS the arena.

## Why This Exists

In the SuperInstance fleet, agents need to coordinate without a central planner telling every agent exactly what to do. A musical jam session is a natural metaphor: musicians listen to each other, respect rules (counterpoint), and collectively create something none could alone. This crate models that coordination pattern using ternary states, where each agent's "note" is a choice between Flat, Rest, and Sharp.

## Core Concepts

- **TernaryNote**: A musical value in {-1, 0, +1}. Flat (-1) = tension/pull, Rest (0) = silence, Sharp (+1) = resolution/push.
- **Voice**: An agent's musical role — has a tendency (bias toward a ternary value) and a sequence of notes to play.
- **ChordProgression**: The harmonic context — a repeating sequence of ternary chords that frames the improvisation.
- **ImprovRule**: Species counterpoint constraints — Parallel (same direction), Contrary (opposite), Free (no constraint), Resolve (toward rest).
- **JamSync**: Tempo synchronization — beats per measure, ticks per beat, keeps all agents on the same timeline.
- **JamSession**: The arena — coordinates voices, progression, and sync into a unified performance with dissonance/consonance scoring.
- **JamMix**: Mixes multiple voices into a single output stream using weighted combination, unanimous agreement, or majority vote.

## Quick Start

```toml
[dependencies]
ternary-jam = "0.1"
```

```rust
use ternary_jam::*;

let progression = ChordProgression::new(vec![[1, 0, -1], [0, 1, 0]]);
let sync = JamSync::new(4, 4);
let mut session = JamSession::new(progression, sync);

let mut voice = Voice::new(0, 1);
voice.add_note(TernaryNote::Sharp);
voice.add_note(TernaryNote::Rest);
voice.add_note(TernaryNote::Flat);
session.add_voice(voice, ImprovRule::Free);

let output = session.run(16);
println!("Output: {:?}", output);
println!("Harmony score: {}", session.harmony_score());
```

## API Overview

| Type | Description |
|------|-------------|
| `TernaryNote` | A single ternary musical value: Flat, Rest, or Sharp |
| `Voice` | An agent's musical role with note sequence and tendency |
| `ChordProgression` | Cycling sequence of ternary chords providing harmonic context |
| `ImprovRule` | Counterpoint constraint for improvisation (Parallel, Contrary, Free, Resolve) |
| `JamSync` | Beat/tick synchronization across agents |
| `JamSession` | The coordinated performance arena combining all elements |
| `JamMix` | Mixer combining multiple voice outputs into one stream |

## How It Works

Each tick, the JamSession advances its sync clock. On beat boundaries, the chord progression advances. Each voice either plays its next queued note or improvises one using its assigned ImprovRule applied to the previous note and the voice's tendency. All voices are mixed (summed, then clamped to {-1, 0, +1}). Pairwise dissonance between voices is tracked — Sharp and Flat sounding simultaneously is dissonant, matching values are consonant.

The improvisation rules implement simplified species counterpoint: Parallel motion encourages continuation, Contrary forces opposition, and Resolve pushes toward the rest state. This creates emergent harmonic behavior from simple local rules.

## Known Limitations

- The mixing strategy is naive (simple summation + clamp). A weighted energy-preserving mix would be more musically accurate.
- Dissonance scoring is binary (dissonant or not) — real harmony has degrees of dissonance.
- No notion of key or scale beyond the ternary abstraction; mapping to actual pitches is outside scope.
- The ImprovRule system is stateless per-tick and doesn't consider longer-range musical structure.

## Use Cases

- **Multi-agent coordination testbed**: Model resource competition where agents choose between three states and must harmonize.
- **Game AI behavior blending**: Multiple AI subsystems (attack, defend, idle) as voices in a jam, mixed into a single action.
- **Consensus visualization**: Watch agents converge or diverge in real-time through harmony scoring.
- **Creative ternary exploration**: Generate ternary sequences with musical structure for artistic or research purposes.

## Ecosystem Context

Part of the SuperInstance ternary crate family. Related to `ternary-music` (musical theory foundations) and `ternary-rhythm` (temporal patterns). This crate focuses on the multi-agent coordination aspect — the jam as an arena for emergent behavior.

## License

MIT
