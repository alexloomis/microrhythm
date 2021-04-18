use microrhythm::resolve::*;

fn test_measure() -> Measure {
    Measure {
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
    }
}

#[test]
fn test_resolve() {
    insta::assert_yaml_snapshot!(resolve(test_measure()))
}

