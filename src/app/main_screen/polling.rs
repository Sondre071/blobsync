use super::{CurrentContainer, MainState, Message};
use crate::shared;

pub fn poll_for_messages(state: &mut MainState) {
    while let Ok(msg) = state.backend.receiver.try_recv() {
        match msg {
            Message::Containers(containers) => {
                shared::println!("%mMessage received: %nContainers");

                state.containers = containers;
            }
            Message::Blobs { container, blobs } => {
                shared::println!("%mMessage received: %nBlobs");
                shared::println!(
                    "%tContainer: %n{}%t, file count: %d{}",
                    container,
                    blobs.len()
                );

                if let Some(current_container) = &mut state.current_container
                    && current_container.name == container
                {
                    current_container.insert_new_blobs(blobs);
                } else {
                    state.current_container = Some(CurrentContainer {
                        name: container,
                        blobs,
                    });
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
