import os

# Меняйте, если ваше расположение файлов отличается от данного
PATH = r"C:\Program Files (x86)\Steam\steamapps\common\dota 2 beta\game\dota\panorama\fonts"

# Аналогично меняете, если вам нужен другой шрифт
need_font_name = "radiance-lightitalic"

def delete_extra_fonts(directory_path, font_name):
    if not os.path.exists(directory_path):
        return

    for file_name in os.listdir(directory_path):
        if font_name not in file_name:
            file_path = os.path.join(directory_path, file_name)
            try:
                if os.path.isfile(file_path):
                    os.remove(file_path)
                    print("Готово")
            except Exception as e:
                print(f"Ошибка при удалении... Ошибка: {e}")


delete_extra_fonts(PATH, need_font_name)
