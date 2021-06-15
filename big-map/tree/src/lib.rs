mod map;

pub mod ic;
pub mod mock;
pub mod storage;

pub use map::*;

#[cfg(test)]
mod test;
