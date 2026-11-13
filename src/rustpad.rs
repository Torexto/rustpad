use crate::buffer::ContentBuffer;
use anyhow::Result;

pub struct Rustpad {
    buffers: Vec<ContentBuffer>,
    active_buffer_index: Option<usize>,
}

impl Default for Rustpad {
    fn default() -> Self {
        Self {
            buffers: Vec::new(),
            active_buffer_index: None,
        }
    }
}

impl Rustpad {
    pub fn buffers(&self) -> &Vec<ContentBuffer> {
        &self.buffers
    }

    pub fn active_buffer_index(&self) -> Option<usize> {
        self.active_buffer_index
    }

    pub fn current_buffer(&self) -> Option<&ContentBuffer> {
        if let Some(active_buffer_index) = self.active_buffer_index {
            if let Some(buffer) = self.buffers.get(active_buffer_index) {
                return Some(buffer);
            }
        }

        None
    }

    pub fn current_buffer_mut(&mut self) -> Option<&mut ContentBuffer> {
        if let Some(active_buffer_index) = self.active_buffer_index {
            if let Some(buffer) = self.buffers.get_mut(active_buffer_index) {
                return Some(buffer);
            }
        }

        None
    }

    pub fn new_buffer(&mut self) {
        self.buffers.push(ContentBuffer::default());
        self.active_buffer_index = Some(self.buffers.len() - 1);
    }

    pub fn set_buffer(&mut self, index: usize) {
        if index < self.buffers.len() {
            self.active_buffer_index = Some(index);
        }
    }

    pub fn remove_buffer(&mut self, index: usize) {
        if index >= self.buffers.len() {
            return;
        }

        self.buffers.remove(index);

        self.active_buffer_index = match self.active_buffer_index {
            Some(active_buffer) if active_buffer > index => Some(active_buffer - 1),
            Some(active_buffer) if active_buffer < index => Some(active_buffer),
            Some(_) if !self.buffers.is_empty() => Some(index.min(self.buffers.len() - 1)),
            _ => None,
        };
    }

    pub fn remove_active_buffer(&mut self) {
        if let Some(active_buffer_index) = self.active_buffer_index {
            self.remove_buffer(active_buffer_index);
        }
    }

    pub fn reload_current_buffer(&mut self) -> Result<()> {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.read()?;
        }

        Ok(())
    }

    pub fn open_file(&mut self) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            let content = std::fs::read_to_string(&path)?;

            let mut buffer = ContentBuffer::from_path(path);

            buffer.content = content;

            self.buffers.push(buffer);
            self.active_buffer_index = Some(self.buffers.len() - 1);
        }

        Ok(())
    }
}
