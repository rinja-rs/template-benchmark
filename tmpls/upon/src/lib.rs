use tmpls::{BigTable, Teams};
use upon::{Engine, Error};

#[derive(Debug)]
pub struct Benchmark {
    engine: Engine<'static>,
}

impl Default for Benchmark {
    fn default() -> Self {
        let mut engine = Engine::new();
        engine
            .add_template("big-table", include_str!("../templates/big-table.html"))
            .unwrap();
        engine
            .add_template("teams", include_str!("../templates/teams.html"))
            .unwrap();
        Self { engine }
    }
}

impl tmpls::Benchmark for Benchmark {
    type Output = Vec<u8>;
    type Error = Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        self.engine
            .template("big-table")
            .render(input)
            .to_writer(output)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        self.engine
            .template("teams")
            .render(input)
            .to_writer(output)
    }
}
