use markup::Render;
use tmpls::{BigTable, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

#[allow(clippy::needless_lifetimes)] // false-positive
impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = std::fmt::Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        markup::define! {
            Tmpl<'a>(input: &'a BigTable) {
                table {
                    @for row in &input.table {
                        tr {
                            @for col in row {
                                td { @col }
                            }
                        }
                    }
                }
            }
        }

        Tmpl { input }.render(output)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        markup::define! {
            Tmpl<'a>(input: &'a Teams) {
                html {
                    head {
                        title { @input.year }
                    }
                    body {
                        h1 { "CLS " @input.year }
                        ul {
                            @for (idx, team) in input.teams.iter().enumerate() {
                                li[class = (idx == 0).then_some("champion")] {
                                    b { @team.name } ": " @team.score
                                }
                            }
                        }
                    }
                }
            }
        }

        Tmpl { input }.render(output)
    }
}
