use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "suln")]
#[command(bin_name = "suln")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'B', long = "before-context", value_name = "NUM")]
    before_context: Option<u64>,

    #[arg(short = 'A', long = "after-context", value_name = "NUM")]
    after_context: Option<u64>,

    #[arg(short = 'C', long = "context", value_name = "NUM")]
    context: Option<u64>,
}

impl Cli {
    pub fn adjust(self) -> (u64, u64) {
        let mut before_context = 0u64;
        let mut after_context = 0u64;

        if let Some(v) = self.context {
            before_context = v;
            after_context = v;
        }

        if let Some(v) = self.before_context {
            before_context = v;
        }

        if let Some(v) = self.after_context {
            after_context = v;
        }

        (before_context, after_context)
    }
}
