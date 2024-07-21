use std::{
    collections::LinkedList,
    fs::{write, File, Metadata, Permissions},
    io::{self, BufRead},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ArranoFile {
    pub path: Option<PathBuf>,
    pub buffer: LinkedList<String>,
    pub data: Option<Metadata>,
}

impl ArranoFile {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_owned();
        let file = File::open(&path)?;
        let data = file.metadata()?;

        let reader = io::BufReader::new(file);
        let mut buffer = LinkedList::new();

        for line in reader.lines() {
            buffer.push_back(line?);
        }

        Ok(Self {
            path: Some(path),
            data: Some(data),
            buffer,
        })
    }

    pub fn new_empty() -> Self {
        let buffer = LinkedList::new();
        Self {
            buffer,
            data: None,
            path: None,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let content: String = self
            .buffer
            .iter()
            .map(|line| format!("{}\n", line))
            .collect();
        let path = self.path.clone().unwrap();
        write(&path, content)
    }
}

pub fn format_permissions(permissions: Permissions, is_directory: bool) -> String {
    let mode = permissions.mode();

    let file_type_char = if is_directory { 'd' } else { '-' };

    let owner_read = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_execute = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_execute = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_execute = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char,
        owner_read,
        owner_write,
        owner_execute,
        group_read,
        group_write,
        group_execute,
        other_read,
        other_write,
        other_execute
    )
}

pub fn is_file(path: impl AsRef<str>) -> bool {
    let path = std::path::Path::new(path.as_ref());
    path.is_file()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn measure_new_arrano_file_time() {
        let _file =
            ArranoFile::new("/home/mikel/Escritorio/rust/arrano/prueba.lua").expect("POLLA");
        assert_eq!(2, 2);
    }
    #[test]
    fn measuere_save_time() {
        let file = ArranoFile::new("/home/mikel/Escritorio/rust/arrano/prueba.lua").expect("POLLA");
        file.save().expect("ERROR SAVE");
        assert_eq!(2, 2);
    }
}
