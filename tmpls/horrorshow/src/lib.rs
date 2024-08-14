use horrorshow::{html, Error, Template};
use tmpls::{BigTable, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        let BigTable { table } = input;
        let tmpl = html! {
            table {
                @for row in table {
                    tr {
                        @for col in row {
                            td : col;
                        }
                    }
                }
            }
        };
        tmpl.write_to_string(output)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let Teams { year, teams } = input;
        let tmpl = html! {
            html {
                head {
                    title : year;
                }
                body {
                    h1 {
                        : "CLS ";
                        : year;
                    }
                    ul {
                        @for (idx, team) in teams.iter().enumerate() {
                            li(class? = (idx == 0).then_some("champion")) {
                                b { : &team.name }
                                : ": ";
                                : team.score;
                            }
                        }
                    }
                }
            }
        };
        tmpl.write_to_string(output)
    }
}
