use std::path::PathBuf;
use crossbeam_channel::Sender;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};

pub struct FileWatcher {
    paths: Vec<PathBuf>,
    channel: Sender<(PathBuf, String)>,
}

impl FileWatcher {
    pub fn new(paths: Vec<PathBuf>, channel: Sender<(PathBuf, String)>) -> Self {
        FileWatcher { paths, channel }
    }

    pub fn run(self) -> notify::Result<()> {
        let tx = self.channel.clone();

        let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                match res {
                    Ok(event) => {
                        // Handle events
                        match &event.kind {
                            EventKind::Create(_) => {
                                for path in &event.paths {
                                    let _ = tx.send((path.clone(), "created".to_string()));
                                }
                            }
                            EventKind::Modify(ModifyKind::Data(_))
                            | EventKind::Modify(ModifyKind::Metadata(_)) => {
                                for path in &event.paths {
                                    let _ = tx.send((path.clone(), "updated".to_string()));
                                }
                            }
                            EventKind::Remove(_) => {
                                for path in &event.paths {
                                    let _ = tx.send((path.clone(), "deleted".to_string()));
                                }
                            }
                            EventKind::Modify(ModifyKind::Name(_)) => {
                                // Rename events: usually two paths (old, new)
                                if event.paths.len() == 2 {
                                    let old = &event.paths[0];
                                    let new = &event.paths[1];
                                    let _ = tx.send((
                                        old.clone(),
                                        format!("renamed to {}", new.display()),
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("watch error: {:?}", e);
                    }
                }
            },
            Config::default(),
        )?;

        for path in &self.paths {
            if path.exists() {
                watcher.watch(path, RecursiveMode::Recursive)?;
            }
        }

        // Keep thread alive
        loop {
            std::thread::park();
        }
    }
}
