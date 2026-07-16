use crate::app::types::{MainState, Message};
use crate::utils;

pub fn poll_for_messages(state: &mut MainState) {
    while let Ok(msg) = state.backend.receiver.try_recv() {
        match msg {
            Message::Containers(containers) => {
                utils::println!("%mMessage received: %nContainers%t");
                utils::println!("Container count: %d{0}", containers.len());

                state.containers = containers;
            }
            Message::Blobs {
                container,
                blobs,
                location,
            } => {
                utils::println!("%mMessage received: %nBlobs");
                utils::println!("%tContainer: %n{}", container);
                utils::println!("%tLocation: %n{}", location);
                utils::println!("%tFile count: %d{}", blobs.len());

                if let Some(current_container) = &mut state.current_container
                    && current_container.name == container
                {
                    current_container.insert_new_blobs(blobs);
                } else {
                    utils::println!(
                        "%wContainer not currently selected. Disregarding the result."
                    );
                }
            }
            Message::BlobWithBytes(blob) => {
                utils::println!("%mMessage received: %nBlobBytes");
                utils::println!("%tFile: %n{}", blob.name);

                state.displayed_blob = Some(blob);
            }
        }

        println!();
    }
}
