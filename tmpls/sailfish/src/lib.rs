use sailfish::runtime::Buffer;
use sailfish::{RenderError, TemplateOnce};
use tmpls::{BigTable, Output, Teams};

#[derive(Debug, Default)]
pub struct Benchmark;

impl tmpls::Benchmark for Benchmark {
    type Output = BufferWrapper;
    type Error = RenderError;

    fn big_table(
        &mut self,
        output: &mut Self::Output,
        input: &BigTable,
    ) -> Result<(), Self::Error> {
        #[derive(TemplateOnce)]
        #[template(path = "big-table.stpl")]
        struct Tmpl<'a> {
            input: &'a BigTable,
        }

        Tmpl { input }.render_once_to(&mut output.0)
    }

    fn teams(&mut self, output: &mut Self::Output, input: &Teams) -> Result<(), Self::Error> {
        #[derive(TemplateOnce)]
        #[template(path = "teams.stpl")]
        struct Tmpl<'a> {
            input: &'a Teams,
        }

        Tmpl { input }.render_once_to(&mut output.0)
    }
}

#[derive(Debug, Default)]
pub struct BufferWrapper(Buffer);

impl Output for BufferWrapper {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.0.as_str().as_bytes()
    }
}
