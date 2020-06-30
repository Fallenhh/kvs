use std::path::PathBuf;
use anyhow::Result;

pub struct KvStore{
}

impl KvStore {

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore {
        })
    }

    pub fn set(&mut self, k: String, v: String) ->Result<()>{
        Ok(())
    }

    pub fn remove(&mut self, k: String) -> Result<()>{
        Ok(())
    }

    pub fn get(&self, k:String) -> Result<Option<String>> {
        Ok(None)
    }
}