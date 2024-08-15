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

impl Default for BigTable {
    fn default() -> Self {
        const SIZE: usize = 100;

        let mut table = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            let mut inner = Vec::with_capacity(SIZE);
            for i in 0..SIZE {
                inner.push(i);
            }
            table.push(inner);
        }
        BigTable { table }
    }
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

impl Default for Teams {
    fn default() -> Self {
        Teams {
            year: 2015,
            teams: vec![
                Team {
                    name: "Jiangsu".into(),
                    score: 43,
                },
                Team {
                    name: "Beijing".into(),
                    score: 27,
                },
                Team {
                    name: "Guangzhou".into(),
                    score: 22,
                },
                Team {
                    name: "Shandong".into(),
                    score: 12,
                },
            ],
        }
    }
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
