mod bool_gene;
mod int_gene;
mod perm_gene;
mod real_gene;

pub use bool_gene::Bool;
pub use int_gene::Int;
pub use perm_gene::Perm;
pub use real_gene::Real;

pub trait Gene {
    type I;
    type V;

    fn new(input: &Self::I) -> Self;
    fn get(&self) -> Self::V;
}
