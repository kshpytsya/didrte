mod error;

pub use error::{DidrteError, DidrteResult};

#[derive(Clone)]
pub struct Runner {
    base_dir: std::path::PathBuf,
    simulate_failure: bool,
}

impl Runner {
    pub fn new(base_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            simulate_failure: false,
        }
    }

    pub fn simulate_failure(mut self) -> Self {
        self.simulate_failure = true;
        self
    }

    pub fn run(self) -> DidrteResult<()> {
        let tests_dir = self.base_dir.join("tests");
        let _test_dirs: Vec<_> = std::fs::read_dir(&tests_dir)
            .map_err(|source| DidrteError::ListTestDir {
                d: tests_dir,
                source,
            })?
            .collect();

        if self.simulate_failure {
            return Err(DidrteError::Failed);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn list_tests_dir_error() {
        let tmp = TempDir::new().unwrap();
        std::fs::File::create(tmp.path().join("tests")).unwrap();

        assert!(matches!(
            Runner::new(tmp.path()).run(),
            DidrteResult::Err(DidrteError::ListTestDir { .. })
        ));
    }

    #[test]
    fn empty_success() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir(tmp.path().join("tests")).unwrap();

        Runner::new(tmp.path()).run().unwrap();
    }

    #[test]
    fn simulated_failure() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir(tmp.path().join("tests")).unwrap();

        assert!(matches!(
            Runner::new(tmp.path()).simulate_failure().run(),
            DidrteResult::Err(DidrteError::Failed)
        ));
    }
}
