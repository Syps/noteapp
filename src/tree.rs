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

pub fn print_tree(root: &str, dir: &Directory) {
    const OTHER_CHILD: &str = "│   ";
    const OTHER_ENTRY: &str = "├── ";
    const FINAL_CHILD: &str = "│\u{00A0}\u{00A0} ";
    const FINAL_ENTRY: &str = "└── ";

    println!("{}", root);
    let (d, f) = visit(dir, "");
    println!("\n{} directories, {} files", d, f);

    fn visit(node: &Directory, prefix: &str) -> (usize, usize) {
        let mut dirs: usize = 1;
        let mut files: usize = 0;
        let mut count = node.entries.len();

        for entry in &node.entries {
            count -= 1;
            let connector = if count == 0 { FINAL_ENTRY } else { OTHER_ENTRY };

            match entry {
                FileTree::DirNode(sub_dir) => {
                    println!("{}{}{}", prefix, connector, sub_dir.name);
                    let new_prefix = format!(
                        "{}{}",
                        prefix,
                        if count == 0 { FINAL_CHILD } else { OTHER_CHILD }
                    );
                    let (d, f) = visit(&sub_dir, &new_prefix);
                    dirs += d;
                    files += f;
                }
                FileTree::LinkNode(symlink) => {
                    println!("{}{}{} -> {}", prefix, connector, symlink.name, symlink.target);
                    files += 1;
                }
                FileTree::FileNode(file) => {
                    println!("{}{}{}", prefix, connector, file.name);
                    files += 1;
                }
            }
        }
        (dirs, files)
    }
}
