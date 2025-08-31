use clap::{App, Arg, SubCommand};
use ts_core::baseline_test::BaselineTestRunner;
use std::path::PathBuf;

pub fn run_baseline_tests() {
    let matches = App::new("TypeScript Baseline Tests")
        .version("1.0")
        .author("TypeScript Rust Team")
        .about("Run baseline tests for TypeScript compiler")
        .subcommand(
            SubCommand::with_name("test")
                .about("Run baseline tests")
                .arg(
                    Arg::with_name("pattern")
                        .short("p")
                        .long("pattern")
                        .value_name("PATTERN")
                        .help("Test name pattern to filter tests")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("test-dir")
                        .long("test-dir")
                        .value_name("DIR")
                        .help("Directory containing test cases")
                        .takes_value(true)
                        .default_value("tests/cases/compiler"),
                )
                .arg(
                    Arg::with_name("baseline-dir")
                        .long("baseline-dir")
                        .value_name("DIR")
                        .help("Directory containing baseline files")
                        .takes_value(true)
                        .default_value("tests/baselines"),
                ),
        )
        .get_matches();

    if let Some(test_matches) = matches.subcommand_matches("test") {
        let test_dir = PathBuf::from(test_matches.value_of("test-dir").unwrap());
        let baseline_dir = PathBuf::from(test_matches.value_of("baseline-dir").unwrap());
        let pattern = test_matches.value_of("pattern");

        let runner = BaselineTestRunner::new(test_dir, baseline_dir);
        let results = runner.run_all_tests(pattern);
        
        let report = runner.generate_report(&results);
        println!("{}", report);
        
        // 如果有失败的测试，退出码为1
        let failed_count = results.iter().filter(|r| !r.passed).count();
        if failed_count > 0 {
            std::process::exit(1);
        }
    }
}