use notify::{RecursiveMode, Result, Watcher};
use std::path::Path;
use tauri::Manager;
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() -> Result<()> {
    let app = tauri::Builder::default()
        .setup(move |_app| Ok(()))
        .build(tauri::generate_context!())
        .expect("error while running tauri builder");

    let handle = app.handle();

    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            handle
                .emit_all(
                    "file-event",
                    Payload {
                        message: "Tauri is awesome!".into(),
                    },
                )
                .unwrap();
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .expect("error creating file watcher");

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("error watching folder");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });

    Ok(())
}
