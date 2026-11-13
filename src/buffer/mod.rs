use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ContentBuffer {
    title: String,
    path: Option<PathBuf>,
    pub(crate) content: String,
}

impl Default for ContentBuffer {
    fn default() -> Self {
        Self {
            title: String::from("untitled"),
            path: None,
            content: String::new(),
        }
    }
}

impl ContentBuffer {
    pub fn from_title(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Self {
            title: path.file_name().unwrap().to_string_lossy().into(),
            path: Some(path),
            ..Self::default()
        }
    }
}

impl ContentBuffer {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn save(&self) {
        if let Some(path) = self.path() {
            std::fs::write(path, &self.content).unwrap();
        }
    }

    pub fn read(&mut self) -> Result<()> {
        if let Some(path) = self.path() {
            let content = std::fs::read(path)?;
            self.content = String::from_utf8(content)?;
        }

        Ok(())
    }
}
