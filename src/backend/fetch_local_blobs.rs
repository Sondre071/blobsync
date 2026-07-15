use super::Backend;
use crate::app::{Blob, Location, Message};
use crate::shared;
use std::path::Path;

use egui::Context;
use walkdir::WalkDir;

impl Backend {
    pub(super) fn dispatch_fetch_local_blobs(&self, ctx: &Context, container: &str) {
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
            let mut blobs = Vec::<Blob>::new();

            for file in WalkDir::new(&root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                // TODO: Stream byte reading and md5 hashing in case of large files.
                let Ok(bytes) = std::fs::read(file.path()) else {
                    let feedback = format!(
                        "Failed to read file: {}, container: {}",
                        file.file_name().to_string_lossy(),
                        &local_container_name
                    );

                    eprintln!("{}", feedback);
                    continue;
                };

                let length = bytes.len() as u64;
                let name = file.file_name().to_string_lossy().to_string();

                let digest = md5::compute(bytes);

                let blob = Blob::new(name, length, None, digest.0, Location::Local);
                blobs.push(blob);
            }

            sender
                .send(Message::Blobs {
                    container: local_container_name,
                    blobs,
                })
                .expect("Failed to fetch local blobs.");

            ctx.request_repaint();
        });
    }
}
