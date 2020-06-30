use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, Seek, SeekFrom};
use serde::{Serialize, Deserialize};
use serde_json;
pub use anyhow::Result;

pub struct KvStore{
    log: File,
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

impl KvStore {

    pub fn open<T: Into<PathBuf>>(path: T) -> Result<KvStore> {
        let mut pathbuf = path.into();
        pathbuf.push("data");
        Ok(KvStore {
            log: OpenOptions::new()
                             .read(true)
                             .append(true)
                             .create(true)
                             .open(pathbuf.as_path())?
        })
    }

    pub fn set(&mut self, k: String, v: String) ->Result<()>{
        let payload = Command::Set{k ,v};
        let serialized_payload = serde_json::to_string(&payload)?;

        writeln!(self.log, "{}", &serialized_payload)?;
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()>{
        self.log.seek(SeekFrom::Start(0))?;
        let br = BufReader::new(&self.log);
        let mut rm_flag = false;
        for line in br.lines() {
            let line = line.unwrap();
            let command: Command = serde_json::from_str(line.as_str())?;
            match command {
                Command::Set{k ,v} => {
                    if k == key {
                       rm_flag = true;
                    }
                }

                Command::Rm{k} => {
                    if k == key {
                       rm_flag = false;
                    }
                }
            }
        }

        if rm_flag {
            let payload = Command::Rm{k: key};
            let serialized_payload = serde_json::to_string(&payload)?;
            writeln!(self.log, "{}", &serialized_payload)?;
        }
        else {
            println!("Key not found");
            panic!()
        }
        Ok(())
    }

    pub fn get(&self, k:String) -> Result<Option<String>> {
        Ok(None)
    }
}