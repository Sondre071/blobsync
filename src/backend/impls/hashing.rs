use super::{Backend, Message};
use crate::shared;

use egui::Context;
use std::path::Path;
use walkdir::WalkDir;

impl Backend {
    pub fn dispatch_local_files_indexing(&self, ctx: &Context, container: &str) {
        let local_container_name = container.replace('-', "_");
        let root = Path::new(&self.account.local_path).join(&local_container_name);
        let sender = self.sender.clone();
        let ctx = ctx.clone();

        if root.try_exists().is_err() {
            shared::println!(
                "%tNo local folder found for container: '%n{}%t', returning.",
                local_container_name
            );

            return;
        }

        self.runtime.spawn_blocking(move || {
            for file in WalkDir::new(&root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let bytes = std::fs::read(file.path()).expect("Unable to read file.");

                let name = file.file_name().to_string_lossy().to_string();

                let digest = md5::compute(bytes);

                sender
                    .send(Message::HashedFile { name, digest })
                    .expect("Failed to hash file.");

                ctx.request_repaint();
            }
        });
    }
}
