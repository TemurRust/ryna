mod stat;
mod analyzer;
mod error;
mod cli;

use clap::Clap;
use analyzer::NginxLogAnalyzer;
use cli::NginxLogAnalyzerCli;

fn main() {
    let cli = NginxLogAnalyzerCli::parse();
    println!("using log fmt file `{}`, type fmt file `{}`, access log `{}`", cli.logfmt, cli.typfmt, cli.acclog);

    let mut analyzer = NginxLogAnalyzer::new();

    let apply_result = analyzer.apply_log_format_files(&cli.logfmt,&cli.typfmt);
    match apply_result {
        Ok(()) => {},
        Err(err) => {
            println!("failed to load file, detail: {}", err);
            return;
        }
    }

    let analyze_result = analyzer.apply_access_log_file(&cli.acclog);
    match analyze_result {
        Ok(()) => {},
        Err(err) => {
            println!("failed to analyze access log, detail: {}", err);
            return;
        }
    }
    println!("{}", analyzer.get_result());
}
