use crate::db;
use notify::Watcher;
use std::path::{Path, PathBuf};

pub struct DirWatcher {
    watcher: notify::ReadDirectoryChangesWatcher,
}

fn handle_event(event: notify::Event) {
    let paths: Vec<PathBuf> = event.paths;
    match event.kind {
        notify::EventKind::Create(_) => db::_add_song(&paths[0]),
        notify::EventKind::Remove(_) => db::_remove_song(&paths[0]),
        notify::EventKind::Modify(kind) => match kind {
            notify::event::ModifyKind::Name(rename_kind) => match rename_kind {
                notify::event::RenameMode::From => db::_remove_song(&paths[0]),
                notify::event::RenameMode::To => db::_add_song(&paths[0]),
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

impl DirWatcher {
    pub fn new(paths: &Vec<PathBuf>) -> Self {
        let mut watcher = notify::recommended_watcher(move |event| match event {
            Ok(event) => handle_event(event),
            Err(e) => panic!("Failed to watch dir: {}", e),
        })
        .expect("Couldn't create file watcher");
        for path in paths {
            watcher.watch(path.as_path(), notify::RecursiveMode::Recursive);
        }

        Self { watcher }
    }

    pub fn add_dir(&mut self, dir: &Path) -> Result<(), notify::Error> {
        self.watcher.watch(dir, notify::RecursiveMode::Recursive)
    }

    pub fn remove_dir(&mut self, dir: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(dir)
    }
}
