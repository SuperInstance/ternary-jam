//! # ternary-jam
//!
//! Musical jam session as multi-agent coordination. Agents improvise within a
//! ternary harmonic framework — competing for harmonic space while cooperating
//! to create consonance. The jam session IS the arena.

#![forbid(unsafe_code)]

/// A ternary musical value: Flat (-1), Rest (0), Sharp (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryNote {
    Flat = -1,
    Rest = 0,
    Sharp = 1,
}

impl TernaryNote {
    pub fn to_i8(self) -> i8 {
        self as i8
    }

    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(TernaryNote::Flat),
            0 => Some(TernaryNote::Rest),
            1 => Some(TernaryNote::Sharp),
            _ => None,
        }
    }

    /// Dissonance contribution: Sharp and Flat together = dissonant.
    pub fn dissonance_with(self, other: Self) -> i8 {
        let a = self.to_i8();
        let b = other.to_i8();
        if a != 0 && b != 0 && a != b { 2 } else { 0 }
    }
}

/// An agent's musical role in the jam.
#[derive(Debug, Clone)]
pub struct Voice {
    /// Unique voice identifier.
    pub id: u32,
    /// Agent's tendency toward Flat/Rest/Sharp (-1, 0, +1).
    pub tendency: i8,
    /// Improvised note sequence.
    pub notes: Vec<TernaryNote>,
    /// Current position in the note sequence.
    pub position: usize,
}

impl Voice {
    pub fn new(id: u32, tendency: i8) -> Self {
        Self { id, tendency: tendency.clamp(-1, 1), notes: Vec::new(), position: 0 }
    }

    /// Play the next note, or Rest if sequence is exhausted.
    pub fn play_next(&mut self) -> TernaryNote {
        if self.position < self.notes.len() {
            let note = self.notes[self.position];
            self.position += 1;
            note
        } else {
            TernaryNote::Rest
        }
    }

    /// Reset playback position.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Append a note to the voice's sequence.
    pub fn add_note(&mut self, note: TernaryNote) {
        self.notes.push(note);
    }

    /// How many notes remain unplayed.
    pub fn remaining(&self) -> usize {
        self.notes.len().saturating_sub(self.position)
    }
}

/// A chord progression built from ternary harmonies.
#[derive(Debug, Clone)]
pub struct ChordProgression {
    /// Each step is a set of ternary values representing simultaneous notes.
    pub steps: Vec<[i8; 3]>,
    /// Current step index.
    pub current: usize,
}

impl ChordProgression {
    pub fn new(steps: Vec<[i8; 3]>) -> Self {
        Self { steps, current: 0 }
    }

    /// Advance to the next chord step, wrapping around.
    pub fn advance(&mut self) -> [i8; 3] {
        let chord = self.steps[self.current];
        self.current = (self.current + 1) % self.steps.len();
        chord
    }

    /// Current chord without advancing.
    pub fn current_chord(&self) -> Option<[i8; 3]> {
        self.steps.get(self.current).copied()
    }

    /// Total number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Harmonic tension of current chord: sum of absolute values.
    pub fn tension(&self) -> i8 {
        self.current_chord().map(|c| c[0].abs() + c[1].abs() + c[2].abs()).unwrap_or(0)
    }

    /// Reset to beginning.
    pub fn reset(&mut self) {
        self.current = 0;
    }
}

/// A species counterpoint constraint for improvisation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImprovRule {
    /// Must move in the same direction as the previous note.
    Parallel,
    /// Must move in the opposite direction.
    Contrary,
    /// No constraint; free improvisation.
    Free,
    /// Must resolve toward Rest (0).
    Resolve,
}

impl ImprovRule {
    /// Apply the rule to generate the next note given previous state.
    pub fn apply(self, prev: TernaryNote, tendency: i8) -> TernaryNote {
        let p = prev.to_i8();
        match self {
            ImprovRule::Parallel => {
                let next = (p + tendency.signum()).clamp(-1, 1);
                TernaryNote::from_i8(next).unwrap_or(TernaryNote::Rest)
            }
            ImprovRule::Contrary => {
                let next = (p - tendency.signum()).clamp(-1, 1);
                TernaryNote::from_i8(next).unwrap_or(TernaryNote::Rest)
            }
            ImprovRule::Free => {
                TernaryNote::from_i8(tendency).unwrap_or(TernaryNote::Rest)
            }
            ImprovRule::Resolve => {
                // Move toward 0
                if p > 0 { TernaryNote::from_i8(p - 1).unwrap_or(TernaryNote::Rest) }
                else if p < 0 { TernaryNote::from_i8(p + 1).unwrap_or(TernaryNote::Rest) }
                else { TernaryNote::Rest }
            }
        }
    }

    /// Check if a transition between two notes obeys this rule.
    pub fn check(self, prev: TernaryNote, next: TernaryNote) -> bool {
        let p = prev.to_i8();
        let n = next.to_i8();
        match self {
            ImprovRule::Parallel => (n - p).signum() != -p.signum() || p == 0 || n == p,
            ImprovRule::Contrary => (n - p).signum() == -p.signum() || p == 0,
            ImprovRule::Free => true,
            ImprovRule::Resolve => n.abs() <= p.abs(),
        }
    }
}

/// Tempo synchronization between agents.
#[derive(Debug, Clone)]
pub struct JamSync {
    /// Beats per measure.
    pub beats_per_measure: u32,
    /// Current beat within the measure (0-indexed).
    pub current_beat: u32,
    /// Global tempo in ticks per beat.
    pub ticks_per_beat: u32,
    /// Accumulated ticks.
    pub tick_counter: u32,
}

impl JamSync {
    pub fn new(beats_per_measure: u32, ticks_per_beat: u32) -> Self {
        Self { beats_per_measure, current_beat: 0, ticks_per_beat, tick_counter: 0 }
    }

    /// Advance by one tick. Returns true when a new beat starts.
    pub fn tick(&mut self) -> bool {
        self.tick_counter += 1;
        if self.tick_counter >= self.ticks_per_beat {
            self.tick_counter = 0;
            self.current_beat = (self.current_beat + 1) % self.beats_per_measure;
            true
        } else {
            false
        }
    }

    /// Current beat index.
    pub fn beat(&self) -> u32 {
        self.current_beat
    }

    /// Is this the downbeat (first beat)?
    pub fn is_downbeat(&self) -> bool {
        self.current_beat == 0 && self.tick_counter == 0
    }

    /// Fraction of the current beat elapsed (0..1).
    pub fn beat_fraction(&self) -> f64 {
        self.tick_counter as f64 / self.ticks_per_beat.max(1) as f64
    }

    /// Total ticks in one full measure.
    pub fn measure_ticks(&self) -> u32 {
        self.beats_per_measure * self.ticks_per_beat
    }

    /// Reset to beginning of measure.
    pub fn reset(&mut self) {
        self.current_beat = 0;
        self.tick_counter = 0;
    }
}

/// The coordinated jam session — the arena where agents perform.
#[derive(Debug, Clone)]
pub struct JamSession {
    /// Participating voices.
    pub voices: Vec<Voice>,
    /// Harmonic context.
    pub progression: ChordProgression,
    /// Timing sync.
    pub sync: JamSync,
    /// Active improv rules per voice (by index).
    pub rules: Vec<ImprovRule>,
    /// Combined output for each tick.
    pub output: Vec<i8>,
    /// Dissonance score accumulated.
    pub dissonance: i64,
    /// Consonance score accumulated.
    pub consonance: i64,
    /// Ticks elapsed.
    pub ticks: u64,
}

impl JamSession {
    pub fn new(progression: ChordProgression, sync: JamSync) -> Self {
        Self {
            voices: Vec::new(),
            progression,
            sync,
            rules: Vec::new(),
            output: Vec::new(),
            dissonance: 0,
            consonance: 0,
            ticks: 0,
        }
    }

    /// Add a voice with an optional improv rule.
    pub fn add_voice(&mut self, voice: Voice, rule: ImprovRule) {
        self.voices.push(voice);
        self.rules.push(rule);
    }

    /// Advance one tick: each voice plays, output is mixed, scores updated.
    pub fn tick(&mut self) -> i8 {
        let new_beat = self.sync.tick();
        if new_beat {
            self.progression.advance();
        }

        let mut mixed: i32 = 0;
        let mut active_count = 0u32;
        let notes: Vec<TernaryNote> = self.voices.iter_mut().enumerate().map(|(i, v)| {
            if v.remaining() == 0 {
                // Improvise based on rule
                let rule = self.rules.get(i).copied().unwrap_or(ImprovRule::Free);
                let prev = v.notes.last().copied().unwrap_or(TernaryNote::Rest);
                rule.apply(prev, v.tendency)
            } else {
                v.play_next()
            }
        }).collect();

        for n in &notes {
            let v = n.to_i8() as i32;
            mixed += v;
            if *n != TernaryNote::Rest {
                active_count += 1;
            }
        }

        // Pairwise dissonance scoring
        for i in 0..notes.len() {
            for j in (i + 1)..notes.len() {
                let d = notes[i].dissonance_with(notes[j]);
                if d > 0 {
                    self.dissonance += d as i64;
                } else {
                    self.consonance += 1;
                }
            }
        }

        // Clamp mix to ternary range
        let result = if mixed > 0 { 1i8 } else if mixed < 0 { -1i8 } else { 0i8 };
        self.output.push(result);
        self.ticks += 1;
        result
    }

    /// Run the session for N ticks.
    pub fn run(&mut self, ticks: u32) -> Vec<i8> {
        let mut results = Vec::with_capacity(ticks as usize);
        for _ in 0..ticks {
            results.push(self.tick());
        }
        results
    }

    /// Current harmony score: consonance minus dissonance.
    pub fn harmony_score(&self) -> i64 {
        self.consonance - self.dissonance
    }

    /// Number of voices in the session.
    pub fn voice_count(&self) -> usize {
        self.voices.len()
    }
}

/// Mixer that combines multiple voice outputs into a single ternary stream.
#[derive(Debug, Clone)]
pub struct JamMix {
    /// Weight for each voice index.
    pub weights: Vec<i8>,
}

impl JamMix {
    pub fn new(weights: Vec<i8>) -> Self {
        Self { weights }
    }

    /// Mix a set of ternary values using weights.
    pub fn mix(&self, values: &[TernaryNote]) -> TernaryNote {
        let mut sum: i32 = 0;
        for (i, v) in values.iter().enumerate() {
            let w = self.weights.get(i).copied().unwrap_or(1) as i32;
            sum += v.to_i8() as i32 * w;
        }
        if sum > 0 { TernaryNote::Sharp } else if sum < 0 { TernaryNote::Flat } else { TernaryNote::Rest }
    }

    /// Unanimous agreement: all non-rest voices have the same value.
    pub fn unanimous(&self, values: &[TernaryNote]) -> bool {
        let non_rest: Vec<_> = values.iter().filter(|&&v| v != TernaryNote::Rest).collect();
        if non_rest.is_empty() { return true; }
        non_rest.windows(2).all(|w| w[0] == w[1])
    }

    /// Majority vote among non-rest values.
    pub fn majority(&self, values: &[TernaryNote]) -> TernaryNote {
        let mut sharp = 0i32;
        let mut flat = 0i32;
        for v in values {
            match v {
                TernaryNote::Sharp => sharp += 1,
                TernaryNote::Flat => flat += 1,
                TernaryNote::Rest => {}
            }
        }
        if sharp > flat { TernaryNote::Sharp }
        else if flat > sharp { TernaryNote::Flat }
        else { TernaryNote::Rest }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ternary_note_roundtrip() {
        for v in [-1i8, 0, 1] {
            assert_eq!(TernaryNote::from_i8(v).unwrap().to_i8(), v);
        }
        assert!(TernaryNote::from_i8(2).is_none());
    }

    #[test]
    fn dissonance_detection() {
        assert_eq!(TernaryNote::Flat.dissonance_with(TernaryNote::Sharp), 2);
        assert_eq!(TernaryNote::Sharp.dissonance_with(TernaryNote::Flat), 2);
        assert_eq!(TernaryNote::Rest.dissonance_with(TernaryNote::Sharp), 0);
        assert_eq!(TernaryNote::Sharp.dissonance_with(TernaryNote::Sharp), 0);
    }

    #[test]
    fn voice_playback() {
        let mut v = Voice::new(0, 1);
        v.add_note(TernaryNote::Sharp);
        v.add_note(TernaryNote::Rest);
        v.add_note(TernaryNote::Flat);
        assert_eq!(v.play_next(), TernaryNote::Sharp);
        assert_eq!(v.play_next(), TernaryNote::Rest);
        assert_eq!(v.play_next(), TernaryNote::Flat);
        assert_eq!(v.play_next(), TernaryNote::Rest); // exhausted
        assert_eq!(v.remaining(), 0);
    }

    #[test]
    fn voice_reset() {
        let mut v = Voice::new(1, -1);
        v.add_note(TernaryNote::Flat);
        v.play_next();
        assert_eq!(v.remaining(), 0);
        v.reset();
        assert_eq!(v.remaining(), 1);
    }

    #[test]
    fn chord_progression_cycles() {
        let mut cp = ChordProgression::new(vec![[1, 0, -1], [0, 1, 0], [-1, 0, 1]]);
        assert_eq!(cp.advance(), [1, 0, -1]);
        assert_eq!(cp.advance(), [0, 1, 0]);
        assert_eq!(cp.advance(), [-1, 0, 1]);
        assert_eq!(cp.advance(), [1, 0, -1]); // wraps
    }

    #[test]
    fn chord_tension() {
        let cp = ChordProgression::new(vec![[1, 1, 1], [0, 0, 0]]);
        assert_eq!(cp.tension(), 3); // current = first chord
    }

    #[test]
    fn improv_rule_free() {
        assert!(ImprovRule::Free.check(TernaryNote::Flat, TernaryNote::Sharp));
        let result = ImprovRule::Free.apply(TernaryNote::Rest, 1);
        assert_eq!(result, TernaryNote::Sharp);
    }

    #[test]
    fn improv_rule_resolve() {
        let r = ImprovRule::Resolve.apply(TernaryNote::Sharp, 0);
        assert_eq!(r, TernaryNote::Rest);
        let r = ImprovRule::Resolve.apply(TernaryNote::Flat, 0);
        assert_eq!(r, TernaryNote::Rest);
        assert!(ImprovRule::Resolve.check(TernaryNote::Sharp, TernaryNote::Rest));
    }

    #[test]
    fn jam_sync_beats() {
        let mut s = JamSync::new(4, 4);
        assert!(s.is_downbeat());
        assert!(!s.tick()); // tick 1
        assert!(!s.tick()); // tick 2
        assert!(!s.tick()); // tick 3
        assert!(s.tick());  // tick 4 = new beat
        assert_eq!(s.beat(), 1);
    }

    #[test]
    fn jam_sync_measure_ticks() {
        let s = JamSync::new(3, 4);
        assert_eq!(s.measure_ticks(), 12);
    }

    #[test]
    fn jam_sync_fraction() {
        let mut s = JamSync::new(4, 4);
        assert_eq!(s.beat_fraction(), 0.0);
        s.tick();
        assert!(s.beat_fraction() > 0.0);
    }

    #[test]
    fn jam_session_basic() {
        let cp = ChordProgression::new(vec![[1, 0, -1]]);
        let sync = JamSync::new(4, 4);
        let mut session = JamSession::new(cp, sync);
        let mut v = Voice::new(0, 1);
        v.add_note(TernaryNote::Sharp);
        v.add_note(TernaryNote::Rest);
        session.add_voice(v, ImprovRule::Free);
        let results = session.run(8);
        assert_eq!(results.len(), 8);
        assert!(session.ticks > 0);
    }

    #[test]
    fn jam_session_harmony_score() {
        let cp = ChordProgression::new(vec![[1, 0, -1]]);
        let sync = JamSync::new(4, 4);
        let mut session = JamSession::new(cp, sync);
        let v1 = Voice::new(0, 1);
        let v2 = Voice::new(1, 1);
        session.add_voice(v1, ImprovRule::Free);
        session.add_voice(v2, ImprovRule::Free);
        session.run(4);
        // Both voices have tendency 1, so should be consonant
        assert!(session.consonance > 0);
    }

    #[test]
    fn jam_session_voice_count() {
        let cp = ChordProgression::new(vec![[0, 0, 0]]);
        let sync = JamSync::new(4, 4);
        let mut session = JamSession::new(cp, sync);
        assert_eq!(session.voice_count(), 0);
        session.add_voice(Voice::new(0, 1), ImprovRule::Free);
        assert_eq!(session.voice_count(), 1);
    }

    #[test]
    fn jam_mix_weighted() {
        let m = JamMix::new(vec![2, 1]);
        let result = m.mix(&[TernaryNote::Sharp, TernaryNote::Flat]);
        assert_eq!(result, TernaryNote::Sharp); // 2*1 + 1*(-1) = 1 > 0
    }

    #[test]
    fn jam_mix_unanimous() {
        let m = JamMix::new(vec![]);
        assert!(m.unanimous(&[TernaryNote::Sharp, TernaryNote::Sharp]));
        assert!(!m.unanimous(&[TernaryNote::Sharp, TernaryNote::Flat]));
        assert!(m.unanimous(&[TernaryNote::Rest, TernaryNote::Rest]));
    }

    #[test]
    fn jam_mix_majority() {
        let m = JamMix::new(vec![]);
        assert_eq!(m.majority(&[TernaryNote::Sharp, TernaryNote::Sharp, TernaryNote::Flat]), TernaryNote::Sharp);
        assert_eq!(m.majority(&[TernaryNote::Rest, TernaryNote::Rest]), TernaryNote::Rest);
    }

    #[test]
    fn voice_clamps_tendency() {
        let v = Voice::new(0, 5);
        assert_eq!(v.tendency, 1);
        let v2 = Voice::new(1, -3);
        assert_eq!(v2.tendency, -1);
    }

    #[test]
    fn chord_progression_reset() {
        let mut cp = ChordProgression::new(vec![[1, 0, 0], [0, 1, 0]]);
        cp.advance();
        cp.advance();
        assert_eq!(cp.current, 0);
        cp.advance();
        assert_eq!(cp.current, 1);
        cp.reset();
        assert_eq!(cp.current, 0);
    }

    #[test]
    fn jam_sync_reset() {
        let mut s = JamSync::new(4, 2);
        s.tick();
        s.tick(); // new beat
        assert_eq!(s.beat(), 1);
        s.reset();
        assert_eq!(s.beat(), 0);
        assert_eq!(s.tick_counter, 0);
    }
}
