include!(concat!(env!("OUT_DIR"), "/templates.rs"));

use tmpls::{BigTable, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = Vec<u8>;
    type Error = std::io::Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        templates::big_table_html(output, input)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        templates::teams_html(output, input)
    }
}
