#!/usr/bin/env python3
"""
Скрипт для загрузки всех файлов assets из GitHub репозитория Egregoria
Использует встроенный список всех файлов (более надёжно и не требует API)
"""

import os
import sys
from pathlib import Path
import urllib.request
import urllib.error
import ssl

# Параметры репозитория
REPO_OWNER = "Uriopass"
REPO_NAME = "Egregoria"
BRANCH = "master"
GITHUB_MEDIA_BASE = f"https://media.githubusercontent.com/media/{REPO_OWNER}/{REPO_NAME}/refs/heads/{BRANCH}"

# Цвета для вывода
class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    RESET = '\033[0m'

def is_lfs_placeholder(file_path: Path) -> bool:
    """Проверяет, является ли файл LFS-плейсхолдером"""
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            first_line = f.readline()
            return "git-lfs" in first_line
    except:
        return False

def get_files_list() -> list:
    """
    Возвращает список всех файлов, которые должны быть в assets
    """
    files = [
        # Шрифты и основные ресурсы
        "assets/SpaceMono-Regular.ttf",
        "assets/font_awesome_solid_900.otf",
        "assets/crates_architecture.jpg",
        "assets/paris_54000.txt",
        
        # Скриншоты
        "assets/screen1.jpg",
        "assets/screen2.jpg",
        "assets/screen3.jpg",
        "assets/screen5.jpg",
        
        # UI основное
        "assets/ui/housebrush.png",
        "assets/ui/curved.png",
        "assets/ui/buildings.png",
        "assets/ui/road.png",
        "assets/ui/road_edit_old.png",
        "assets/ui/terraform.png",
        "assets/ui/trainstation.png",
        "assets/ui/traintool.png",
        "assets/ui/bulldozer_old.png",
        
        # Локализация
        "assets/i18n/en.json",
        "assets/i18n/ru.json",
    ]
    
    # Иконки UI
    ui_icons = [
        "roadedit_auto.png", "roadedit_back_turn.png", "roadedit_crosswalk.png",
        "roadedit_forbidden.png", "roadedit_left_turn.png", "roadedit_light.png",
        "roadedit_no_light.png", "roadedit_roundabout.png", "roadedit_stop_sign.png",
        "roadtypes_avenue.png", "roadtypes_avenue_1way.png", "roadtypes_drive.png",
        "roadtypes_drive_1way.png", "roadtypes_highway.png", "roadtypes_highway_1way.png",
        "roadtypes_rail.png", "roadtypes_rail_1way.png", "roadtypes_street.png",
        "roadtypes_street_1way.png", "select_triangle_under.png", "snap_angle.png",
        "snap_grid.png", "snap_notting.png", "terraforming_erode.png",
        "terraforming_level.png", "terraforming_radius_large.png",
        "terraforming_radius_medium.png", "terraforming_radius_small.png",
        "terraforming_raise_lower.png", "terraforming_slope.png", "terraforming_smooth.png",
        "terraforming_speed_large.png", "terraforming_speed_low.png",
        "terraforming_speed_medium.png", "toolbar_bulldozer.png", "toolbar_companies.png",
        "toolbar_curved_road.png", "toolbar_housetool.png", "toolbar_road_edit.png",
        "toolbar_straight_road.png", "toolbar_terraform.png", "toolbar_train.png",
        "height_reference_decline.png", "height_reference_ground.png",
        "height_reference_incline.png", "height_reference_start.png",
        "no_power.png", "bread.jpg", "carcass.jpg", "cereal.jpg", "flour.jpg",
        "flower.jpg", "gold.jpg", "meat.jpg", "metal.jpg", "tree-log.jpg",
        "vegetable.jpg", "wood-plank.jpg", "wool.jpg",
    ]
    
    files.extend([f"assets/ui/icons/{icon}" for icon in ui_icons])
    
    # Шейдеры
    shaders = [
        "alpha_discard.frag.wgsl", "atmosphere.wgsl", "atmosphere_cubemap.frag.wgsl",
        "background.wgsl", "dither.wgsl", "fog.wgsl", "instanced_mesh.vert.wgsl",
        "lit_mesh.vert.wgsl", "mipmap.wgsl", "pixel.frag.wgsl", "render_params.wgsl",
        "shadow.wgsl", "shadow_depth_write.frag.wgsl", "spritebatch.vert.wgsl",
        "ssao.wgsl", "to_cubemap.vert.wgsl", "tonemap.wgsl", "ui_blur.wgsl",
        "water.frag.wgsl", "compute/texture_write.wgsl", "heightmap/calc_normals.wgsl",
        "heightmap/heightmap.frag.wgsl", "heightmap/heightmap.vert.wgsl",
        "heightmap/resample.wgsl", "heightmap/unpack.wgsl", "pbr/brdf_convolution.wgsl",
        "pbr/convolute_diffuse_irradiance.frag.wgsl", "pbr/equirectangular_to_cubemap.frag.wgsl",
        "pbr/render.wgsl", "pbr/sample.wgsl", "pbr/specular_prefilter.frag.wgsl",
    ]
    
    files.extend([f"assets/shaders/{shader}" for shader in shaders])
    
    # Спрайты
    sprites = [
        "animal_farm.png", "arrow_one_way.png", "blue_noise_512.png", "cement.jpg",
        "cereal_farm.png", "cliff.jpg", "cloth_factory.png", "clothes_store.png",
        "crosswalk.png", "dirt.jpg", "florist.png", "foundry.png", "furniture_store.png",
        "grass.jpg", "hightech_facility.png", "hightech_store.png", "horticulturalist.png",
        "iron_mine.png", "lumber_yard.png", "meat_facility.png", "noise.png",
        "oil_pump.png", "palette.png", "path_not_found.png", "petrol_refinery.png",
        "polyester_refinery.png", "rare_metal_mine.png", "slaughterhouse.png", "starfield.png",
        "supermarket.png", "textile_processing_facility.png", "vegetable_farm.png",
        "wavy.jpeg", "woodmill.png", "wool_farm.png",
    ]
    
    files.extend([f"assets/sprites/{sprite}" for sprite in sprites])
    
    # Звуки
    sounds = [
        "calm_wind.ogg", "car_engine.ogg", "car_loop.ogg", "forest.ogg",
        "music1.ogg", "music2.ogg", "road_lay.ogg",
    ]
    
    files.extend([f"assets/sounds/{sound}" for sound in sounds])
    
    # Модели
    models = [
        "bakery.glb", "cinema.glb", "coal_power_plant.glb", "external_trading.glb",
        "flour_factory.glb", "passenger-emu-front.glb", "passenger-emu-middle.glb",
        "passenger-emu-rear.glb", "pedestrian.glb", "pine.glb", "rail_freight_station.glb",
        "roadedit_auto.glb", "salad.glb", "simple_car.glb", "solarpanel.glb", "sphere.glb",
        "stop_sign.glb", "streetlamp.glb", "traffic_light_green.glb", "traffic_light_orange.glb",
        "traffic_light_red.glb", "train.glb", "truck.glb", "wagon.glb",
        "wagon_freight.glb", "wheat_up.glb",
    ]
    
    files.extend([f"assets/models/{model}" for model in models])
    
    return sorted(set(files))

def download_file(file_path: str, base_dir: Path, github_base: str) -> str:
    """
    Загружает файл с GitHub
    Возвращает: 'OK', 'SKIP', 'FAILED'
    """
    file_path_obj = base_dir / file_path
    
    # Проверяем локальный файл
    if file_path_obj.exists():
        size = file_path_obj.stat().st_size
        
        # Проверяем, это LFS-плейсхолдер?
        if size < 300 and is_lfs_placeholder(file_path_obj):
            # Заменяем LFS плейсхолдер
            pass
        else:
            # Файл нормальный, пропускаем
            return 'SKIP'
    
    # Если файл не существует или это LFS плейсхолдер, скачиваем его
    if not file_path_obj.exists():
        print(f"  {Colors.BLUE}[RESTORE]{Colors.RESET} {file_path} ... ", end='', flush=True)
    elif file_path_obj.stat().st_size < 300 and is_lfs_placeholder(file_path_obj):
        print(f"  {Colors.YELLOW}[REPLACE]{Colors.RESET} {file_path} ... ", end='', flush=True)
    else:
        return 'SKIP'
    
    relative_path = file_path.replace('\\', '/')
    url = f"{github_base}/{relative_path}"
    
    try:
        # Создаём директорию
        file_path_obj.parent.mkdir(parents=True, exist_ok=True)
        
        # Создаём SSL контекст без проверки сертификата (для macOS)
        ssl_context = ssl.create_default_context()
        ssl_context.check_hostname = False
        ssl_context.verify_mode = ssl.CERT_NONE
        
        # Скачиваем файл с использованием urlopen
        with urllib.request.urlopen(url, context=ssl_context) as response:
            with open(file_path_obj, 'wb') as f:
                f.write(response.read())
        
        # Проверяем результат
        size = file_path_obj.stat().st_size
        
        if is_lfs_placeholder(file_path_obj):
            print(f"{Colors.RED}✗ FAILED (LFS placeholder){Colors.RESET}")
            return 'FAILED'
        else:
            print(f"{Colors.GREEN}✓{Colors.RESET}")
            return 'OK'
            
    except urllib.error.HTTPError as e:
        print(f"{Colors.RED}✗ FAILED (HTTP {e.code}){Colors.RESET}")
        return 'FAILED'
    except Exception as e:
        print(f"{Colors.RED}✗ FAILED ({str(e)}){Colors.RESET}")
        return 'FAILED'

def main():
    # Проверяем аргументы командной строки
    base_path = None
    if len(sys.argv) > 1:
        base_path = Path(sys.argv[1]).resolve()  # Преобразуем в абсолютный путь
        if not base_path.exists():
            print(f"{Colors.RED}✗ Ошибка: папка не существует: {base_path}{Colors.RESET}")
            return 1
        if not (base_path / "assets").exists():
            print(f"{Colors.RED}✗ Ошибка: в папке {base_path} не найдена папка 'assets'{Colors.RESET}")
            return 1
    else:
        # Пытаемся автоматически найти папку Egregoria
        current = Path.cwd()
        if (current / "assets").exists():
            base_path = current
        else:
            # Ищем папку с Cargo.toml (признак rust проекта)
            for parent in [current] + list(current.parents):
                if (parent / "assets").exists() and (parent / "Cargo.toml").exists():
                    base_path = parent
                    break
        
        if base_path is None:
            print(f"{Colors.RED}✗ Ошибка: папка 'assets' не найдена!{Colors.RESET}")
            print(f"\nИспользование:")
            print(f"  python3 download_assets.py")
            print(f"  или")
            print(f"  python3 download_assets.py /path/to/Egregoria")
            return 1
    
    # Меняем рабочую директорию
    os.chdir(base_path)
    
    print(f"{Colors.BLUE}{'='*60}{Colors.RESET}")
    print(f"Загрузка файлов assets из GitHub")
    print(f"Репозиторий: {REPO_OWNER}/{REPO_NAME}")
    print(f"Ветка: {BRANCH}")
    print(f"Папка: {base_path}")
    print(f"{Colors.BLUE}{'='*60}{Colors.RESET}")
    print()
    
    # Получаем список файлов для загрузки
    files_list = get_files_list()
    
    print(f"Файлов для проверки: {len(files_list)}\n")
    
    # Загружаем файлы
    total = len(files_list)
    success = 0
    skipped = 0
    failed = 0
    failed_files = []
    
    for idx, file_path in enumerate(files_list, 1):
        result = download_file(file_path, base_path, GITHUB_MEDIA_BASE)
        
        if result == 'OK':
            success += 1
        elif result == 'SKIP':
            skipped += 1
        elif result == 'FAILED':
            failed += 1
            failed_files.append(file_path)
    
    # Итоги
    print()
    print(f"{Colors.BLUE}{'='*60}{Colors.RESET}")
    print("Итоги загрузки:")
    print(f"  Всего файлов: {total}")
    print(f"  {Colors.GREEN}Загружено: {success}{Colors.RESET}")
    print(f"  Пропущено (уже есть): {skipped}")
    if failed > 0:
        print(f"  {Colors.RED}Ошибки: {failed}{Colors.RESET}")
    
    if failed_files:
        print(f"\n{Colors.RED}Файлы с ошибками:{Colors.RESET}")
        for file in failed_files[:10]:
            print(f"  - {file}")
        if len(failed_files) > 10:
            print(f"  ... и ещё {len(failed_files) - 10}")
    
    print(f"{Colors.BLUE}{'='*60}{Colors.RESET}")
    
    if failed == 0:
        print(f"{Colors.GREEN}✓ Готово!{Colors.RESET}")
        return 0
    else:
        print(f"{Colors.YELLOW}⚠ Завершено с ошибками{Colors.RESET}")
        return 1

if __name__ == "__main__":
    sys.exit(main())
