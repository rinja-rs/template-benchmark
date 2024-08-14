use minijinja::{Environment, Error};
use tmpls::{BigTable, Teams};

#[derive(Debug)]
pub struct Benchmark {
    env: Environment<'static>,
}

impl Default for Benchmark {
    fn default() -> Self {
        let mut env = Environment::new();
        env.add_template("big-table", include_str!("../templates/big-table.html"))
            .unwrap();
        env.add_template("teams", include_str!("../templates/teams.html"))
            .unwrap();
        Self { env }
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
        self.env
            .get_template("big-table")
            .unwrap()
            .render_to_write(input, output)
            .map(|_| ())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        self.env
            .get_template("teams")
            .unwrap()
            .render_to_write(input, output)
            .map(|_| ())
    }
}
