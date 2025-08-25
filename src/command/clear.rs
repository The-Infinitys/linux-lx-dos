use crate::modules::app::gui::Gui;
use crate::LxDosError;
use std::fs;

pub fn clear() -> Result<(), LxDosError> {
    let paths_to_remove = Gui::get_icon_cache_files();

    if paths_to_remove.is_empty() {
        println!("No cache files to clear.");
        return Ok(());
    }

    println!("The following cache files will be removed:");
    for path in &paths_to_remove {
        println!("- {}", path.display());
    }

    for path in paths_to_remove {
        match fs::remove_file(&path) {
            Ok(_) => println!("Removed {}", path.display()),
            Err(e) => eprintln!("Failed to remove {}: {}", path.display(), e),
        }
    }

    println!("Cache clearing process finished.");

    Ok(())
}