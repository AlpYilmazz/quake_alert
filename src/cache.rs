use std::{
    fs::File,
    io::{BufReader, BufWriter},
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{process::QuakeData, util::Sink};

#[derive(Default)]
pub struct QuakeDataCache(pub Vec<QuakeData>);
impl Deref for QuakeDataCache {
    type Target = Vec<QuakeData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for QuakeDataCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl QuakeDataCache {
    pub fn load(_path: impl AsRef<Path>) -> Self {
        todo!()
    }

    pub fn save(&self) -> Result<(), ()> {
        Err(())
    }
}

pub struct LastQuake(pub Option<QuakeData>);
impl Deref for LastQuake {
    type Target = Option<QuakeData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LastQuake {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let Ok(file) = File::open(path) else {
            return Self(None);
        };
        let reader = BufReader::new(file);
        Self(ron::de::from_reader(reader).ok().flatten())
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ()> {
        let file = File::create(path).map_err(Sink::sink)?;
        let writer = BufWriter::new(file);
        ron::ser::to_writer(writer, &self.0).map_err(Sink::sink)
    }
}
