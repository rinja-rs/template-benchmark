use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait Benchmark: Default {
    type Output: Output;
    type Error: std::error::Error;

    fn big_table(&mut self, output: &mut Self::Output, input: &BigTable)
    -> Result<(), Self::Error>;

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct BigTable {
    pub table: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Teams {
    pub year: u16,
    pub teams: Vec<Team>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub score: u8,
}

pub trait Output: Default {
    fn clear(&mut self);
    fn as_bytes(&self) -> &[u8];
}

impl Output for String {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Output for Vec<u8> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}
