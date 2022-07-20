use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Case {
    pub description: String,
    pub schema: serde_json::Value,
    pub tests: Vec<Test>,
}

impl Case {
    pub fn run(&self, implementations: &Vec<String>) -> CaseResults {
        CaseResults {
            // case: self,
            test_results: self
                .tests
                .iter()
                .map(|test| test.run(&self, implementations))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub description: String,
    pub instance: serde_json::Value,
    pub valid: Option<bool>,
}

impl Test {
    pub fn run(&self, case: &Case, implementations: &Vec<String>) -> TestResults {
        let expected = match self.valid {
            Some(true) => format!(" (valid)"),
            Some(false) => format!(" (invalid)"),
            None => format!(""),
        };
        let results_by_id = HashMap::from_iter(
            implementations
                .iter()
                .map(|name| (name.to_owned(), ImplementationTestResult { valid: true })),
        );
        // TODO: When table-ified, preserve order (values() is arbitrary).
        let display = results_by_id
            .values()
            .map(|result| if result.valid { "valid" } else { "invalid" });
        println!(
            "{} > {}: {} / {}{} – {}",
            case.description,
            self.description,
            case.schema,
            self.instance,
            expected,
            display.collect::<Vec<_>>().join(", "),
        );
        TestResults {
            // test: self,
            implementations: results_by_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CaseResults {
    // #[serde(borrow)]
    // pub case: &'a Case,
    pub test_results: Vec<TestResults>,
}

#[derive(Serialize, Deserialize)]
pub struct TestResults {
    // #[serde(borrow)]
    // pub test: &'a Test,
    pub implementations: HashMap<String, ImplementationTestResult>,
}

#[derive(Serialize, Deserialize)]
pub struct ImplementationTestResult {
    pub valid: bool,
}
