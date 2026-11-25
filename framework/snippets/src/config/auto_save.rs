use std::{
    io::Write,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::Serialize;

/// RAII wrapper that serialises its value to a TOML file on drop.
///
/// Saving can be disabled by constructing with [`AutoSave::no_save`] or by
/// calling [`AutoSave::disable`], which is useful in tests where you don't want
/// side-effects on disk.
pub struct AutoSave<T: Serialize> {
    value: T,
    /// `None` means saving is disabled.
    path: Option<PathBuf>,
}

impl<T: Serialize> AutoSave<T> {
    /// Wraps `value` and saves to `path` on drop.
    pub fn new(value: T, path: impl Into<PathBuf>) -> Self {
        AutoSave {
            value,
            path: Some(path.into()),
        }
    }

    /// Wraps `value` without saving on drop.
    pub fn no_save(value: T) -> Self {
        AutoSave { value, path: None }
    }

    /// Wraps the default value without saving on drop.
    pub fn no_save_default() -> Self
    where
        T: Default,
    {
        AutoSave::no_save(T::default())
    }

    /// Disables saving for this instance.
    pub fn disable(&mut self) {
        self.path = None;
    }

    /// Serialises the value to the configured path immediately.
    ///
    /// Returns `Ok(())` if saving is disabled (no-op) or if the write
    /// succeeds. Returns `Err` on serialisation or I/O failure.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.path {
            let content = toml::to_string(&self.value)?;
            let mut file = std::fs::File::create(path)?;
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }
}

impl<T: Serialize> Deref for AutoSave<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: Serialize> DerefMut for AutoSave<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Serialize> Drop for AutoSave<T> {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("AutoSave: failed to write state: {e}");
        }
    }
}
