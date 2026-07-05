use std::path::Path;
use regex::Regex;

struct LoxExpectations {
    expected_stdout: Vec<String>,
    expects_compile_error: bool,
}

fn parse_lox_file(content: &str) -> LoxExpectations {
    let output_re = Regex::new(r"// expect: ?(.*)").unwrap();
    let error_re = Regex::new(r"// [line \d+] Error").unwrap();

    let mut expected_stdout = Vec::new();
    let mut expects_compile_error = false;

    for line in content.lines() {
        if let Some(caps) = output_re.captures(line) {
            expected_stdout.push(caps[1].to_string());
        }

        if error_re.is_match(line) {
            expects_compile_error = true;
        }
    }

    LoxExpectations {
        expected_stdout,
        expects_compile_error,
    }
}

fn test_lox_script(path: &Path, content: String) -> datatest_stable::Result<()> {
    let expectations = parse_lox_file(&content);

    // match interpreter::run_string {  }

    todo!()
}