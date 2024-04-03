use std::{cmp::Ordering, ffi::OsStr, fs, io::Result, path::PathBuf};

/*
From https://www.georgevreilly.com/blog/2023/01/23/TreeInRust1WalkDirectories.html
*/

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub metadata: fs::Metadata,
}

#[derive(Debug)]
pub struct Symlink {
    pub name: String,
    pub target: String,
    pub metadata: fs::Metadata,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub entries: Vec<FileTree>,
}

#[derive(Debug)]
pub enum FileTree {
    DirNode(Directory),
    FileNode(File),
    LinkNode(Symlink),
}

pub fn is_not_hidden_name(name: &str) -> bool {
    return !name.starts_with(".");
}

fn path_file_name_str(path_buf: &PathBuf) -> String {
    return path_buf.file_name().unwrap().to_str().unwrap().into();
}

pub fn sort_by_name(a: &fs::DirEntry, b: &fs::DirEntry) -> Ordering {
    let a_name: String = path_file_name_str(&a.path());
    let b_name: String = path_file_name_str(&b.path());

    return a_name.cmp(&b_name);
}

pub fn dir_walk(
    root: &PathBuf,
    filter: fn(name: &str) -> bool,
    compare: fn(a: &fs::DirEntry, b: &fs::DirEntry) -> Ordering,
) -> Result<Directory> {
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(root)?
        .filter_map(|result| result.ok())
        .collect();

    entries.sort_by(compare);

    let mut directory: Vec<FileTree> = Vec::with_capacity(entries.len());

    for e in entries {
        let path = e.path();
        let name = path_file_name_str(&path);

        if !filter(&name) {
            continue;
        };
        let metadata = path.metadata()?;
        let node = match path {
            path if path.is_dir() => {
                FileTree::DirNode(dir_walk(&root.join(name), filter, compare)?)
            }
            path if path.is_symlink() => FileTree::LinkNode(Symlink {
                name: name.clone(),
                target: fs::read_link(path).unwrap().to_string_lossy().to_string(),
                metadata: metadata,
            }),
            path if path.is_file() => FileTree::FileNode(File {
                name: name.clone(),
                metadata: metadata,
            }),
            _ => unreachable!(),
        };

        directory.push(node);
    }

    let name = root
        .file_name()
        .unwrap_or(OsStr::new("."))
        .to_str()
        .unwrap()
        .into();

    Ok(Directory {
        name: name,
        entries: directory,
    })
}
