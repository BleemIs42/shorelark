use rand::{prelude::SliceRandom, RngCore};

use crate::genetic_algorithm::Individual;

pub struct RouletteWheelSelection;

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chromosome::Chromosome;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug)]
    pub struct TestIndividual {
        fitness: f32,
    }

    #[cfg(test)]
    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }
    impl Individual for TestIndividual {
        fn chromosome(&self) -> &Chromosome {
            panic!("not supported for TestIndividual")
        }
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }
    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = vec![
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];
        let actual_histogram: BTreeMap<i32, _> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;

                histogram
            });
        let expected_histogram = maplit::btreemap! {
            1 => 102,
            2 => 198,
            3 => 301,
            4 => 399,
        };
        assert_eq!(actual_histogram, expected_histogram);
    }
}
