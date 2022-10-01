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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_none_only() {
        let c = Cli {
            before_context: None,
            after_context: None,
            context: None,
        };
        let (b, a) = c.adjust();
        assert_eq!(b, 0);
        assert_eq!(a, 0);
    }

    #[test]
    fn test_adjust_before_context_only() {
        let c = Cli {
            before_context: Some(1),
            after_context: None,
            context: None,
        };
        let (b, a) = c.adjust();
        assert_eq!(b, 1);
        assert_eq!(a, 0);
    }

    #[test]
    fn test_adjust_after_context_only() {
        let c = Cli {
            before_context: None,
            after_context: Some(1),
            context: None,
        };
        let (b, a) = c.adjust();
        assert_eq!(b, 0);
        assert_eq!(a, 1);
    }

    #[test]
    fn test_adjust_context_only() {
        let c = Cli {
            before_context: None,
            after_context: None,
            context: Some(2),
        };
        let (b, a) = c.adjust();
        assert_eq!(b, 2);
        assert_eq!(a, 2);
    }

    #[test]
    fn test_adjust_all_fields() {
        let c = Cli {
            before_context: Some(1),
            after_context: Some(2),
            context: Some(3),
        };
        let (b, a) = c.adjust();
        assert_eq!(b, 1);
        assert_eq!(a, 2);
    }
}
