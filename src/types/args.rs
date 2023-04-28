use clap::Parser;

use super::language::SupportedLanguage;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub language: SupportedLanguage,
    #[arg(short, long, help = "Path to output file (default: STDOUT)")]
    pub output: Option<String>,
    #[arg(short, long, help = "Path to directory to scan (default: .)")]
    pub dir: Option<String>,
    #[arg(short, long, help = "Namespace to scan (default: 'default')")]
    pub namespace: Option<String>,
}
