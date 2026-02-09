#!/bin/bash

# Скрипт для загрузки всех файлов assets из GitHub
# Использует GitHub raw content URL для media-файлов
# Пример: https://media.githubusercontent.com/media/Uriopass/Egregoria/refs/heads/master/assets/ui/icons/roadedit_auto.png

set -e

REPO_OWNER="Uriopass"
REPO_NAME="Egregoria"
BRANCH="master"
GITHUB_MEDIA_BASE="https://media.githubusercontent.com/media/${REPO_OWNER}/${REPO_NAME}/refs/heads/${BRANCH}"

# Папки для загрузки (исключаем generated, models_opt и т.д.)
ASSET_FOLDERS=(
    "assets/ui"
    "assets/ui/icons"
    "assets/sprites"
    "assets/sounds"
    "assets/shaders"
    "assets/models"
    "assets/i18n"
)

# Счётчики
TOTAL=0
SUCCESS=0
FAILED=0
FAILED_FILES=""

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "============================================"
echo "Загрузка файлов assets из GitHub"
echo "Репозиторий: ${REPO_OWNER}/${REPO_NAME}"
echo "Ветка: ${BRANCH}"
echo "============================================"
echo ""

# Функция для загрузки файла
download_file() {
    local file_path="$1"
    local url="${GITHUB_MEDIA_BASE}/${file_path}"
    
    # Создаём директорию если её нет
    local dir=$(dirname "$file_path")
    mkdir -p "$dir"
    
    # Проверяем, существует ли файл локально и его размер
    if [ -f "$file_path" ]; then
        local local_size=$(stat -f%z "$file_path" 2>/dev/null || echo "0")
        # Если файл маленький (< 300 байт), скорее всего это LFS-плейсхолдер
        if [ "$local_size" -lt 300 ]; then
            echo -e "${YELLOW}[REPLACE]${NC} $file_path (размер: $local_size байт)"
            echo "DOWNLOAD"
            return 0
        else
            # Файл уже скачан, пропускаем
            echo -e "${GREEN}[SKIP]${NC} $file_path (уже существует, размер: $local_size байт)"
            echo "SKIP"
            return 0
        fi
    fi
    
    echo -ne "[DOWNLOAD] $file_path ... "
    
    # Скачиваем файл
    if curl -s -L --create-dirs -o "$file_path" "$url" 2>/dev/null; then
        local size=$(stat -f%z "$file_path" 2>/dev/null || echo "0")
        
        # Проверяем, не LFS-плейсхолдер ли это
        if grep -q "version https://git-lfs.github.com/spec/v1" "$file_path" 2>/dev/null; then
            echo -e "${RED}✗ FAILED (LFS placeholder)${NC}"
            echo "FAILED"
        else
            echo -e "${GREEN}✓ OK${NC} (размер: $size байт)"
            echo "OK"
        fi
    else
        echo -e "${RED}✗ FAILED (download error)${NC}"
        echo "FAILED"
    fi
}

# Проходим по всем файлам в папке assets/
echo "Поиск файлов для загрузки..."
echo ""

# Находим все файлы в assets/ (исключаем некоторые папки)
while IFS= read -r file; do
    if [ -n "$file" ]; then
        TOTAL=$((TOTAL + 1))
        result=$(download_file "$file")
        last_line=$(echo "$result" | tail -n 1)
        
        case "$last_line" in
            OK)
                SUCCESS=$((SUCCESS + 1))
                ;;
            SKIP)
                SUCCESS=$((SUCCESS + 1))
                ;;
            DOWNLOAD)
                # Повторно скачиваем
                actual_result=$(download_file "$file")
                if echo "$actual_result" | grep -q "^OK$"; then
                    SUCCESS=$((SUCCESS + 1))
                else
                    FAILED=$((FAILED + 1))
                    FAILED_FILES="${FAILED_FILES}\n  - $file"
                fi
                ;;
            FAILED)
                FAILED=$((FAILED + 1))
                FAILED_FILES="${FAILED_FILES}\n  - $file"
                ;;
        esac
    fi
done < <(find assets -type f \
    -not -path "*/generated/*" \
    -not -path "*/models_opt/*" \
    -not -path "*/.DS_Store" \
    -not -path "*/.git/*")

# Итоги
echo ""
echo "============================================"
echo "Итоги загрузки:"
echo "  Всего файлов: $TOTAL"
echo -e "  ${GREEN}Успешно: $SUCCESS${NC}"
echo -e "  ${RED}Ошибки: $FAILED${NC}"

if [ $FAILED -gt 0 ]; then
    echo -e "\n${RED}Файлы с ошибками:${FAILED_FILES}${NC}"
fi

echo "============================================"

# Выход с кодом 0, если все прошло хорошо
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ Загрузка завершена успешно!${NC}"
    exit 0
else
    echo -e "${RED}✗ Некоторые файлы не удалось загрузить${NC}"
    exit 1
fi
