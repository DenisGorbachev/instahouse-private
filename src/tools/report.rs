use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::ops::Range;

use crate::{Seed, State, StateValidationErrors};
use derive_getters::Getters;
use derive_more::{Error, From, Into};
use derive_new::new;
use fmt_derive::Display;
use stub_macro::stub;
use time::OffsetDateTime;

#[derive(new, Getters, From, Into, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Report {
    pub seeds: Range<u64>,
    pub steps: u64,
    pub average_revenue: i64,
}

impl Report {}

impl Report {
    #[allow(clippy::result_large_err)]
    pub fn create(seeds: Range<u64>, steps: u64) -> Result<Self, ReportCreateError> {
        let mut average_revenue: f64 = 0.0;
        let mut i: u64 = 1;

        for seed in seeds.clone() {
            // We can also use a longer seed if necessary
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
            let now = OffsetDateTime::UNIX_EPOCH;

            let mut state = State::create(now, &mut rng);

            let result = state.run_n(&mut rng, steps);

            if let Err(errors) = result {
                return Err(StateRunError::new(seed, state, errors).into());
            }

            let current_revenue = stub!(u64);
            average_revenue += (current_revenue as f64 - average_revenue) / i as f64;
            i += 1;
        }

        Ok(Self {
            seeds,
            steps,
            average_revenue: average_revenue as i64,
        })
    }
}

#[derive(Error, Display, From, Eq, PartialEq, Clone, Debug)]
pub enum ReportCreateError {
    StateRun(StateRunError),
}

#[derive(new, Error, Display, Eq, PartialEq, Clone, Debug)]
pub struct StateRunError {
    seed: Seed,
    state: State,
    errors: StateValidationErrors,
}
