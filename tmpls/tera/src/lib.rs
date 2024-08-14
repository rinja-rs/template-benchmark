use tera::{Context, Error, Tera};
use tmpls::{BigTable, Teams};

#[derive(Debug)]
pub struct Benchmark {
    tera: Tera,
}

impl Default for Benchmark {
    fn default() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template("big-table", include_str!("../templates/big-table.html"))
            .unwrap();
        tera.add_raw_template("teams", include_str!("../templates/teams.html"))
            .unwrap();
        Self { tera }
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
        let context = Context::from_serialize(input)?;
        self.tera.render_to("big-table", &context, output)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let context = Context::from_serialize(input)?;
        self.tera.render_to("teams", &context, output)
    }
}
