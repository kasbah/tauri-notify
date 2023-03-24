use notify::{RecursiveMode, Result, Watcher};
use std::path::Path;

fn main() -> Result<()> {
    let mut watcher1 = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            // does get printed
            println!("watcher1 event: {:?}", event);
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .expect("error creating file watcher");

    watcher1
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("error watching folder");

    tauri::Builder::default()
        .setup(|_app| {
            let mut watcher2 = notify::recommended_watcher(move |res| match res {
                Ok(event) => {
                    // does not get printed
                    println!("watcher2 event: {:?}", event);
                }
                Err(e) => println!("watch error: {:?}", e),
            })
            .expect("error creating file watcher");

            watcher2
                .watch(Path::new("."), RecursiveMode::Recursive)
                .expect("error watching folder");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri builder");

    Ok(())
}
