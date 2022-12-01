use std::path::PathBuf;

pub struct AnyStubs(pub Vec<PathBuf>);

impl From<PathBuf> for AnyStubs {
    fn from(path: PathBuf) -> Self {
        Self(vec![path])
    }
}

impl From<Vec<PathBuf>> for AnyStubs {
    fn from(paths: Vec<PathBuf>) -> Self {
        Self(paths)
    }
}

impl From<String> for AnyStubs {
    fn from(path: String) -> Self {
        Self(vec![PathBuf::from(path)])
    }
}

impl From<Vec<String>> for AnyStubs {
    fn from(paths: Vec<String>) -> Self {
        Self(paths.iter().map(PathBuf::from).collect())
    }
}

impl From<&'_ str> for AnyStubs {
    fn from(path: &'_ str) -> Self {
        Self(vec![PathBuf::from(path)])
    }
}

impl From<Vec<&'_ str>> for AnyStubs {
    fn from(paths: Vec<&'_ str>) -> Self {
        Self(paths.iter().map(PathBuf::from).collect())
    }
}
