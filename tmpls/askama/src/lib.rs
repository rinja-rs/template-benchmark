use std::ops::Deref;

use askama::{Error, Template};
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
        #[derive(Template)]
        #[template(path = "big-table.html")]
        struct Tmpl<'a>(&'a BigTable);

        impl Deref for Tmpl<'_> {
            type Target = BigTable;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        Tmpl(input).render_into(output)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        #[derive(Template)]
        #[template(path = "teams.html")]
        struct Tmpl<'a>(&'a Teams);

        impl Deref for Tmpl<'_> {
            type Target = Teams;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        Tmpl(input).render_into(output)
    }
}
