use super::{Backend, Message};

use std::sync::Arc;


use egui::Context;

use walkdir::WalkDir;

impl Backend {
    pub fn dispatch_local_files_indexing(&self, ctx: &Context, container: &str) {
        let path = self.account.local_path.clone();
        let container: Arc<str> = Arc::from(container);
        let sender = self.sender.clone();
        let ctx = ctx.clone();

        self.runtime.spawn_blocking(move || {
            for file in WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let bytes = std::fs::read(file.path()).expect("Unable to read file.");

                let name = file.file_name().to_string_lossy().to_string();

                let digest = md5::compute(bytes);

                sender
                    .send(Message::HashedFile {
                        name,
                        container: Arc::clone(&container),
                        digest,
                    })
                    .expect("Failed to hash file.");

                ctx.request_repaint();
            }
        });
    }
}
