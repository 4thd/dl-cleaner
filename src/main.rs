use std::collections::HashSet;
use std::fs;
use std::path::Path;

extern crate env_logger;
extern crate log;
use env_logger::Env;
use log::{error, info, warn};

fn list_files_recursively(
    dir_path: &Path,
    multimedia_extensions: &HashSet<&str>,
) -> Result<Vec<i32>, std::io::Error> {
    let mut stats: Vec<i32> = vec![0, 0];

    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let is_media_file = is_media(&path, multimedia_extensions);
                info!("File: {:?}", path);
                stats[0] += 1;

                if is_media_file {
                    if let Err(err) = fs::remove_file(&path) {
                        error!("Error, failed to delete file: {:?}", err);
                    } else {
                        warn!("File deleted: : {:?}", path);
                    }
                }
            } else if path.is_dir() {
                info!("Folder : {:?}", path);
                stats[1] += 1;
                list_files_recursively(&path, multimedia_extensions)?;
            }
        }
    } else {
        warn!("Le chemin est vide, veuillez vérifier.");
    }
    Ok(stats)
}

fn is_media(path: &Path, multimedia_extensions: &HashSet<&str>) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return multimedia_extensions.contains(ext_str);
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();

    info!("Start programm");
    let multimedia_extensions: HashSet<&str> = [
        "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "mp3", "wav", "flac", "aac", "ogg",
        "wma", "jpg", "jpeg", "png", "gif", "bmp", "tiff", "svg", "pdf", "epub", "swf", "m4a",
        "srt",
    ]
    .iter()
    .cloned()
    .collect();

    let dir_path = std::env::args()
        .nth(1)
        .ok_or("Please provide a file name")?;

    let dir_path = Path::new(&dir_path);

    match list_files_recursively(dir_path, &multimedia_extensions) {
        Ok(stats) => {
            info!("Files: {}", stats[0]);
            info!("Folders: {}", stats[1]);
        }
        Err(error) => {
            error!("Erreur {:?}", error);
        }
    }
    Ok(())
}
