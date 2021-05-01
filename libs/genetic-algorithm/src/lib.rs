// Use for chromosome.rs `type IntoIter = impl Iterator<Item = f32>`
#![feature(min_type_alias_impl_trait)]

mod chromosome;
mod crossover;
mod genetic_algorithm;
mod mutation;
mod section;

pub use genetic_algorithm::GeneticAlgorithm;
