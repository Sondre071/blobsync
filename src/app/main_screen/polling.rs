use super::{MainState, Message};
use crate::shared;

pub fn poll_for_messages(state: &mut MainState) {
    while let Ok(msg) = state.backend.receiver.try_recv() {
        match msg {
            Message::Containers(containers) => {
                shared::println!("%mMessage received: %nContainers%t");
                shared::println!("Container count: %d{0}", containers.len());

                state.containers = containers;
            }
            Message::Blobs {
                container,
                blobs,
                location,
            } => {
                shared::println!("%mMessage received: %nBlobs");
                shared::println!("%tContainer: %n{}", container);
                shared::println!("%tLocation: %n{}", location);
                shared::println!("%tFile count: %d{}", blobs.len());

                if let Some(current_container) = &mut state.current_container
                    && current_container.name == container
                {
                    current_container.insert_new_blobs(blobs);
                } else {
                    shared::println!(
                        "%wContainer not currently selected. Disregarding the result."
                    );
                }
            }
            Message::BlobWithBytes(blob) => {
                shared::println!("%mMessage received: %nBlobBytes");
                shared::println!("%tFile: %n{}", blob.name);

                state.displayed_blob = Some(blob);
            }
        }

        println!();
    }
}
