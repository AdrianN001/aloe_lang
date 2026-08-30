use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Erstellt die benötigten Unterverzeichnisse relativ zu `base` und gibt den vollständigen Pfad
/// für die Ziel-HTML-Datei zurück.
///
/// # Beispiel
/// Wenn `base = "/output/dir"` und `file_path = "./lib/module/file.aloe"`,
/// werden die Unterverzeichnisse `/output/dir/lib/module` erstellt
/// und der Pfad `/output/dir/lib/module/file.html` zurückgegeben.
pub fn prepare_html_path(
    base: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
) -> io::Result<PathBuf> {
    let rel_path = file_path.as_ref();

    // Führende './' oder '/' entfernen, um sauberes Joinen mit base zu gewährleisten
    let rel_path = rel_path
        .strip_prefix("./")
        .or_else(|_| rel_path.strip_prefix("."))
        .unwrap_or(rel_path);

    // Dateiendung auf 'html' ändern
    let html_rel_path = rel_path.with_extension("html");

    // Zielpfad zusammensetzen
    let target_path = base.as_ref().join(html_rel_path);

    // Alle benötigten übergeordneten Verzeichnisse erstellen
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(target_path)
}
