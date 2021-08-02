// [[file:../spdkit.note::*base][base:1]]
/// Simulated annealing.
#[derive(Clone, Debug)]
pub struct Annealer {
    pub(crate) temperature_high: f64,
    pub(crate) temperature_low: f64,
    cooling_rate: f64,
    temperature: Option<f64>,
}

impl Default for Annealer {
    fn default() -> Self {
        Self {
            temperature_high: 5000.0,
            temperature_low: 2000.0,
            cooling_rate: 0.92,
            temperature: None,
        }
    }
}

impl Annealer {
    /// Construct Annealer with temperature high `th` and temperate low `tl`.
    pub fn new(th: f64, tl: f64) -> Self {
        assert!(th > tl, "temperature_low is high than temperature_high!");

        Self {
            temperature_high: th,
            temperature_low: tl,
            ..Default::default()
        }
    }

    /// Set cooling rate.
    pub fn cooling_rate(mut self, r: f64) -> Self {
        assert!(
            r > 0.0 && r < 1.0,
            "cooling rate should be a number in range 0..1"
        );

        self.cooling_rate = r;
        self
    }

    /// Reset tempeature to initial state.
    pub fn reset(&mut self) {
        self.temperature = Some(self.temperature_high);
    }

    /// Return an iterator over temperature.
    pub fn start(&mut self) -> impl Iterator<Item = f64> + '_ {
        std::iter::from_fn(move || {
            let temp: &mut f64 = self.temperature.get_or_insert(self.temperature_high);
            *temp *= self.cooling_rate;

            if *temp <= self.temperature_low {
                None
            } else {
                Some(*temp)
            }
        })
    }
}

#[test]
fn test_annealer() {
    let mut ann = Annealer::new(500.0, 100.0);

    for x in ann.start() {
        dbg!(x);
    }
}
// base:1 ends here
