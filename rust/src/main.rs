fn main() {
    let test_measure: Measure = Measure {
        mix: 0.5,
        beats: vec![
            Pattern {
                length: 2.0,
                delay: 0.0,
                subdiv: None,
            },
            Pattern {
                length: 1.0,
                delay: -0.2,
                subdiv: Some((
                    0.4,
                    vec![
                        Pattern {
                            length: 2.0,
                            delay: 0.3,
                            subdiv: None,
                        },
                        Pattern {
                            length: 1.0,
                            delay: 0.1,
                            subdiv: Some((
                                0.6,
                                vec![
                                    Pattern {
                                        length: 2.0,
                                        delay: 0.0,
                                        subdiv: None,
                                    },
                                    Pattern {
                                        length: 2.0,
                                        delay: 0.0,
                                        subdiv: None,
                                    },
                                ],
                            )),
                        },
                    ],
                )),
            },
            Pattern {
                length: 1.0,
                delay: 0.0,
                subdiv: None,
            },
        ],
    };
    for i in resolve(test_measure) {
        println!("{:.3}", i)
    }
}

type Durations = Vec<f64>;

#[derive(Clone)]
struct Pattern {
    length: f64,
    delay: f64,
    subdiv: Option<(f64, Vec<Pattern>)>,
}

struct Measure {
    mix: f64,
    beats: Vec<Pattern>,
}

// What's the right way to do this?
fn normalize(vec: Durations) -> Durations {
    let denom: f64 = vec.iter().sum();
    vec.iter().map(|x| x / denom).collect()
}

fn mix_durations(patterns: &[Pattern], mix: f64) -> Durations {
    patterns
        .iter()
        .map(|x| 1.0 + mix * (x.length - 1.0))
        .collect()
}

// delayer([1f64, 2, 3], -0.25, 2) = -0.5
// i.e. it gives the change in duration to the prev beat
// and negative change for the current beat
fn delayer(times: &[f64], delay: f64, index: usize) -> f64 {
    if delay < 0.0 {
        delay * times[index - 1]
    } else {
        delay * times[index]
    }
}

// Note that the delay of the first beat is ignored.
fn apply_delays(patterns: &[Pattern], times: Durations) -> Durations {
    let deltas: Vec<f64> = (1..patterns.len())
        .map(|i| delayer(&times, patterns[i].delay, i))
        .collect();
    let deltas_add: Vec<f64> = deltas.iter().chain(&vec![0f64]).cloned().collect();
    let deltas_sub: Vec<f64> = vec![0f64].iter().chain(&deltas).cloned().collect();
    (0..patterns.len())
        .map(|i| times[i] + deltas_add[i] - deltas_sub[i])
        .collect()
}

fn resolve_subdiv(mix: f64, patterns: &[Pattern]) -> Durations {
    normalize(apply_delays(patterns, mix_durations(patterns, mix)))
}

fn resolve_patterns(ratio: f64, pattern: &Pattern) -> Durations {
    match &pattern.subdiv {
        None => vec![ratio],
        Some((mix, patterns)) => {
            let ratios: Vec<f64> = resolve_subdiv(*mix, &patterns)
                .iter()
                .map(|x| ratio * x)
                .collect();
            (0..patterns.len())
                .map(|i| resolve_patterns(ratios[i], &patterns[i]))
                .flatten()
                .collect()
        }
    }
}

fn resolve(measure: Measure) -> Durations {
    let ratios: Vec<f64> = resolve_subdiv(measure.mix, &measure.beats);
    (0..measure.beats.len())
        .map(|i| resolve_patterns(ratios[i], &measure.beats[i]))
        .flatten()
        .collect()
}

