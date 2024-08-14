use tinytemplate::error::Error;
use tinytemplate::TinyTemplate;
use tmpls::{BigTable, Teams};

pub struct Benchmark {
    tt: TinyTemplate<'static>,
}

impl Default for Benchmark {
    fn default() -> Self {
        let mut tt = TinyTemplate::new();
        tt.add_template("big-table", include_str!("../templates/big-table.html"))
            .unwrap();
        tt.add_template("teams", include_str!("../templates/teams.html"))
            .unwrap();
        Self { tt }
    }
}

impl tmpls::Benchmark for Benchmark {
    type Output = String;
    type Error = Error;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        *output = self.tt.render("big-table", input)?;
        Ok(())
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        *output = self.tt.render("teams", input)?;
        Ok(())
    }
}
