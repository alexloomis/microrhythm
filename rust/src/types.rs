pub type Durations = Vec<f64>;

#[derive(Clone)]
pub struct Pattern {
    pub length: f64,
    pub delay: f64,
    pub subdiv: Option<(f64, Vec<Pattern>)>,
}

pub struct Measure {
    pub mix: f64,
    pub beats: Vec<Pattern>,
}

