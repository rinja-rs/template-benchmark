use handlebars::{Context, Handlebars, RenderError};
use tmpls::{BigTable, Teams};

pub struct Benchmark {
    handlebars: Handlebars<'static>,
}

impl Default for Benchmark {
    fn default() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("big-table", include_str!("../templates/big-table.html"))
            .unwrap();
        handlebars
            .register_template_string("teams", include_str!("../templates/teams.html"))
            .unwrap();
        Self { handlebars }
    }
}

impl tmpls::Benchmark for Benchmark {
    type Output = Vec<u8>;
    type Error = RenderError;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        let context = Context::wraps(input)?;
        self.handlebars
            .render_with_context_to_write("big-table", &context, output)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        let context = Context::wraps(input)?;
        self.handlebars
            .render_with_context_to_write("teams", &context, output)
    }
}
