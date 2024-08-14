use std::convert::Infallible;

use maud::html;
use tmpls::{BigTable, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = Infallible;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        *output = html! {
            table {
                @for row in &input.table {
                    tr {
                        @for col in row {
                            td { (col) }
                        }
                    }
                }
            }
        }
        .0;
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        *output = html! {
            html {
                head {
                    title { (input.year) }
                }
                body {
                    h1 { "CLS " (input.year) }
                    ul {
                        @for (idx, team) in input.teams.iter().enumerate() {
                            li.champion[idx == 0] {
                                b { (team.name) } ": " (team.score)
                            }
                        }
                    }
                }
            }
        }
        .0;
        Ok(())
    }
}
