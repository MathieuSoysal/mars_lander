use crate::genetics::pheno;

use super::iterlimit::*;
use pheno::Fitness;

#[derive(Copy, Clone, Debug)]
pub struct EarlyStopper<F: Fitness> {
    delta: F,
    previous: F,
    iter_limit: IterLimit,
}

impl<F: Fitness> EarlyStopper<F> {
    pub fn new(delta: F, n_iters: u64) -> EarlyStopper<F> {
        EarlyStopper {
            delta,
            previous: F::zero(),
            iter_limit: IterLimit::new(n_iters),
        }
    }

    pub fn update(&mut self, fitness: F) {
        if self.previous.abs_diff(&fitness) < self.delta {
            self.previous = fitness;
            self.iter_limit.inc();
        } else {
            self.iter_limit.reset();
        }
    }

    pub fn reached(&self) -> bool {
        self.iter_limit.reached()
    }
}
