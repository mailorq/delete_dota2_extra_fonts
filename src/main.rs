use std::fs;
use std::path::Path;

fn main() {
    // Меняйте, если ваше расположение файлов отличается от данного
    const PATH: &str = r"C:\Program Files (x86)\Steam\steamapps\common\dota 2 beta\game\dota\panorama\fonts";

    // Аналогично меняете, если вам нужен другой шрифт
    const NEED_FONT_NAME: &str = "radiance-lightitalic";

    delete_extra_fonts(PATH, NEED_FONT_NAME);
}

fn delete_extra_fonts(directory_path: &str, font_name: &str) {
    let path = Path::new(directory_path);

    if !path.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();

                if file_path.is_file() {
                    if let Some(file_name_str) = file_path.file_name().and_then(|n| n.to_str()) {

                        if !file_name_str.contains(font_name) {
                            match fs::remove_file(&file_path) {
                                Ok(_) => println!("Удален файл {:?}", file_path),
                                Err(e) => println!("Ошибка при удалении {:?}... Ошибка: {}", file_path, e),
                            }
                        }
                    }
                }
            }
        }
    }
}
