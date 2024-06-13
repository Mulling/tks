use std::fmt::Display;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug)]
pub struct KernelInfo {
    pub version: KernelVersion,
    pub kwd: PathBuf,
}

#[derive(Debug, Default)]
pub struct KernelVersion {
    pub version: u16,
    pub patchlevel: u16,
    pub sublevel: u16,
    pub extraversion: Option<String>,
    pub name: String,
}

impl KernelVersion {
    fn new(kwd: &Path) -> Result<KernelVersion, io::Error> {
        let mut kv = KernelVersion::default();

        let file = fs::read(kwd.with_file_name("Makefile"))?;

        for line in file.lines() {
            let line = line?;

            if line.starts_with('#') {
                continue;
            }

            if line.starts_with("VERSION") {
                kv.version = Self::get_val(&line).unwrap().trim().parse().unwrap();
                continue;
            }

            if line.starts_with("PATCHLEVEL") {
                kv.patchlevel = Self::get_val(&line).unwrap().trim().parse().unwrap();
                continue;
            }

            if line.starts_with("SUBLEVEL") {
                kv.sublevel = Self::get_val(&line).unwrap().trim().parse().unwrap();
                continue;
            }

            if line.starts_with("EXTRAVERSION") {
                kv.extraversion = Some(Self::get_val(&line).unwrap().trim().to_string());
                continue;
            }

            if line.starts_with("NAME") {
                kv.name = Self::get_val(&line).unwrap().trim().to_string();
                continue;
            }

            if !kv.name.is_empty() {
                break;
            }
        }

        Ok(kv)
    }

    fn get_val(line: &str) -> Option<&str> {
        if let Some(i) = line.find('=') {
            let (_, val) = line.split_at(i + 1);

            return Some(val);
        }
        None
    }
}

impl Display for KernelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}{}",
            self.version,
            self.patchlevel,
            self.extraversion
                .as_ref()
                .unwrap_or(&format!(".{}", self.sublevel))
        )
    }
}

impl KernelInfo {
    pub fn new() -> Result<Option<KernelInfo>, io::Error> {
        let kwd = get_kernel_dir()?.unwrap();

        let kv = KernelVersion::new(&kwd)?;

        println!("{kv:?}");

        todo!();
    }
}

pub(crate) fn get_kernel_dir() -> Result<Option<PathBuf>, io::Error> {
    let mut wd = env::current_dir()?;

    loop {
        for entry in fs::read_dir(wd.as_path())? {
            let entry = entry?;

            if entry.file_name() == ".git" && entry.file_type()?.is_dir() {
                return Ok(Some(wd));
            }
        }

        if let Some(parent) = wd.parent() {
            wd = parent.to_path_buf();
        } else {
            break;
        }
    }

    Ok(None)
}
