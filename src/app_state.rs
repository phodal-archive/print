use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::model::file_tree::FileEntry;

use druid::{Data, Lens};
use std::fs::DirEntry;
use std::{fs, io};

use crate::support::directory;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Data, Lens)]
pub struct AppState {
    pub title: String,
    pub workspace: Workspace,
    pub params: Params,
    pub entry: FileEntry,

    #[serde(default)]
    pub current_file: Option<Arc<Path>>,
    #[serde(default)]
    pub current_dir: Option<Arc<Path>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            workspace: Default::default(),
            params: Default::default(),
            entry: Default::default(),
            current_file: None,
            current_dir: None,
        }
    }
}
impl AppState {
    pub fn set_file(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        let string = match fs::read_to_string(path.as_ref().unwrap()) {
            Ok(str) => str,
            Err(_) => {
                return;
            }
        };

        self.workspace.input_text = string;

        self.current_file = path;
        self.save_print_config();
    }

    pub fn text(&mut self) -> String {
        return self.workspace.input_text.clone();
    }

    // todo: add save project config
    pub fn save_print_config(&mut self) {
        directory::save_config(self);
    }

    pub fn set_dir(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        if let Some(dir) = &path {
            self.entry = path_to_tree(self.title.clone(), dir);
            log::info!("open dir: {:?}", dir);
        }

        self.current_dir = path;
        self.save_print_config();
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    if !entry.path().is_dir() {
        return false;
    }

    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn path_to_tree(title: String, dir: &Arc<Path>) -> FileEntry {
    let mut root = FileEntry::new(title);

    let _result = visit_dirs(dir, 0, &mut root, dir);

    root
}

fn visit_dirs(dir: &Path, depth: usize, node: &mut FileEntry, base_dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        let entry_set = fs::read_dir(dir)?; // contains DirEntry
        let mut entries = entry_set
            .filter_map(|v| match v {
                Ok(dir) => {
                    if is_hidden(&dir) {
                        return None;
                    }
                    Some(dir)
                }
                Err(_) => None,
            })
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

        for (_index, entry) in entries.iter().enumerate() {
            let path = entry.path();

            if path.is_dir() {
                let depth = depth + 1;
                let relative_path = path.strip_prefix(base_dir).unwrap();
                let entry = &mut FileEntry::new(format!("{}", relative_path.display()));
                entry.is_dir = true;
                visit_dirs(&path, depth, entry, base_dir)?;
                node.children.push(entry.to_owned());
            } else {
                let entry1 = FileEntry::from_path(path);
                node.children.push(entry1);
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Data, Lens)]
pub struct Workspace {
    pub input_text: String,
}

impl Workspace {}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            input_text: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Data, Lens)]
pub struct Params {
    pub debug_layout: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            debug_layout: false,
        }
    }
}
