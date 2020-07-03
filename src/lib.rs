use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, Seek, SeekFrom};
use std::collections::{BTreeMap};

use serde::{Serialize, Deserialize};
use serde_json::Deserializer;
pub use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvStoreError {
    #[error("Key not found")]
    KeyNotFound,
}

#[derive(Serialize, Deserialize, Debug)]
enum Command{
    Set {
        k: String,
        v: String,
    },
    // Get {
    //     k: String,
    // },
    Rm {
        k: String,
    }
}

struct CommandOffset {
    from: u64,
    len: u64, 
}

pub struct KvStore{
    path: PathBuf,
    log: File,
    index: BTreeMap<String, CommandOffset>,
}

impl KvStore {

    pub fn open<T: Into<PathBuf>>(path: T) -> Result<KvStore> {
        let mut pathbuf = path.into();
        pathbuf.push("data");
        let mut index = BTreeMap::new();
        let mut log = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(pathbuf.as_path())?;

        KvStore::load(&mut log, &mut index)?;
        Ok(KvStore {
            path: pathbuf,
            log,
            index,
        })
    }

    fn load(logfile: &mut File, index: &mut BTreeMap<String, CommandOffset>) -> Result<()> {
        let mut start = logfile.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(logfile).into_iter::<Command>();
        while let Some(cmd) = stream.next() {
            let end = stream.byte_offset() as u64;
            match cmd? {
                Command::Set{k, ..} => {
                    index.insert(k, CommandOffset{from: start, len: end - start});
                },

                Command::Rm{k} => {
                    index.remove(&k);
                }
            }
            start = end;
        }
        Ok(())
    }

    pub fn set(&mut self, k: String, v: String) ->Result<()>{
        let cmd = Command::Set{k ,v};
        let serialized_payload = serde_json::to_string(&cmd)?;
        let start = self.log.seek(SeekFrom::End(0))?;
        write!(self.log, "{}", &serialized_payload)?;
        let len  = self.log.seek(SeekFrom::End(0))?;
        if let Command::Set{k, ..} = cmd {
            self.index.insert(k, CommandOffset{from: start, len: len});
        }
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()>{
        if self.index.contains_key(&key) {
            let cmd = Command::Rm {k: key};
            let serialized_payload = serde_json::to_string(&cmd)?;
            let start = self.log.seek(SeekFrom::End(0))?;
            write!(self.log, "{}", &serialized_payload)?;
            let len  = self.log.seek(SeekFrom::End(0))?;
            if let Command::Rm{k} = cmd {
                self.index.insert(k, CommandOffset{from: start, len: len});
            }
            Ok(())
        }
        else {
            Err(anyhow!("Key not found"))
        }
    }

    pub fn get(&mut self, key:String) -> Result<Option<String>> {
        if let Some(cmd_off) = self.index.get(&key) {
            self.log.seek(SeekFrom::Start(cmd_off.from))?;
            let mut stream = Deserializer::from_reader(&mut self.log).into_iter::<Command>();
            let cmd = stream.next().expect("Internal Error");
            if let Command::Set{v, ..} = cmd? {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        }
        else {
            Ok(None)
        }
    }
}