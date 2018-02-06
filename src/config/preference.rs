use std::env;
use std::fs;
use std::io;
use std::path;

use atomicwrites;
use serde_json;

use errors::*;


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserPreference {
    token: Option<String>,
}


fn home_dir() -> path::PathBuf {
    match env::home_dir() {
        Some(p) => p,
        // TODO
        None => panic!("cannot get home directory"),
    }
}


// TODO: use native config paths
fn config_prefix() -> path::PathBuf {
    let mut tmp = home_dir();
    tmp.push(".qn-changelog");
    tmp
}


fn config_path() -> path::PathBuf {
    let mut tmp = config_prefix();
    tmp.push("config.json");
    tmp
}


fn ensure_dir(p: &path::Path) -> io::Result<()> {
    if !p.exists() {
        fs::create_dir_all(p)
    } else {
        Ok(())
    }
}


impl UserPreference {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }

        let f = fs::File::open(path)?;
        let result = serde_json::from_reader(io::BufReader::new(f))?;

        Ok(result)
    }

    pub fn token<'a>(&'a self) -> Option<&'a str> {
        // https://stackoverflow.com/a/31234028
        self.token.as_ref().map(String::as_ref)
    }

    pub fn set_token<T: AsRef<str>>(&mut self, token: T) {
        self.token = Some(token.as_ref().to_string());
    }

    pub fn save(&self) -> Result<()> {
        let body = serde_json::to_vec_pretty(self)?;

        let prefix = config_prefix();
        ensure_dir(&prefix)?;

        let path = config_path();
        let af = atomicwrites::AtomicFile::new(&path, atomicwrites::AllowOverwrite);
        use std::io::Write;
        af.write(|f| f.write_all(&body))?;

        Ok(())
    }
}
