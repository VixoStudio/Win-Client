//! UI language for the built-in menus.
//!
//! The client ships one string table: English labels are authored inline in
//! the menu builders, and non-English languages are applied as exact-match
//! lookups at draw/measure time in the menu overlay renderer, so layout and
//! text stay consistent without threading a locale through every builder.

use std::sync::atomic::{AtomicU8, Ordering};

/// Available UI languages. Index is persisted in `Settings::language`.
pub const LANGUAGES: &[(&str, &str)] = &[("en_us", "English"), ("ru_ru", "Русский")];

static LANGUAGE: AtomicU8 = AtomicU8::new(0);

pub fn set_language(index: u8) {
    LANGUAGE.store(index.min(LANGUAGES.len() as u8 - 1), Ordering::Relaxed);
}

pub fn language() -> u8 {
    LANGUAGE.load(Ordering::Relaxed)
}

/// The locale code sent to servers in `ClientInformation`.
pub fn locale() -> &'static str {
    LANGUAGES[language() as usize].0
}

/// Exact-match translation of a menu label. Dynamic strings (player names,
/// server MOTDs, numbers) never match and pass through unchanged.
pub fn translate(text: &str) -> &str {
    if language() == 0 {
        return text;
    }
    for (en, ru) in RU {
        if *en == text {
            return ru;
        }
    }
    text
}

const RU: &[(&str, &str)] = &[
    // Title / main menu.
    ("Singleplayer", "Одиночная игра"),
    ("Multiplayer", "Сетевая игра"),
    ("Minecraft Realms", "Realms"),
    ("Friends", "Друзья"),
    ("Sign in to use friends", "Войдите для друзей"),
    ("Change Language", "Сменить язык"),
    ("Accessibility Settings", "Специальные возможности"),
    ("Options...", "Настройки..."),
    ("Quit Game", "Выйти из игры"),
    ("WinModing", "WinModing"),
    ("Coming soon", "Скоро"),
    ("Done", "Готово"),
    ("Cancel", "Отмена"),
    ("Save", "Сохранить"),
    ("Back", "Назад"),
    ("Edit", "Изменить"),
    ("Delete", "Удалить"),
    ("Select", "Выбрать"),
    ("Play", "Играть"),
    ("Create", "Создать"),
    ("Search", "Поиск"),
    ("Reload", "Перезагрузить"),
    ("OK", "OK"),
    ("Yes", "Да"),
    ("No", "Нет"),
    // Options root.
    ("Options", "Настройки"),
    ("Language...", "Язык..."),
    ("Language", "Язык"),
    ("Chat Settings...", "Настройки чата..."),
    ("Skin Customization...", "Внешний вид..."),
    ("Music & Sounds...", "Музыка и звуки..."),
    ("Video Settings...", "Настройки графики..."),
    ("Controls...", "Управление..."),
    ("Resource Packs...", "Ресурспаки..."),
    ("Accessibility Settings...", "Специальные возможности..."),
    ("Telemetry Data...", "Данные телеметрии..."),
    ("Telemetry Data", "Данные телеметрии"),
    ("Credits & Attribution...", "Авторы и благодарности..."),
    ("Online...", "Онлайн..."),
    ("FOV: Normal", "Угол обзора: Обычный"),
    ("FOV: Quake Pro", "Угол обзора: Quake Pro"),
    // Video.
    ("Video Settings", "Настройки графики"),
    ("Graphics: Fancy", "Графика: Детальная"),
    ("Graphics: Fast", "Графика: Быстрая"),
    ("Smooth Lighting: Maximum", "Плавное освещение: Максимум"),
    ("Smooth Lighting: Fancy", "Плавное освещение: Детально"),
    ("Smooth Lighting: OFF", "Плавное освещение: ВЫКЛ"),
    ("Clouds: Fancy", "Облака: Детальные"),
    ("Clouds: Fast", "Облака: Быстрые"),
    ("Clouds: OFF", "Облака: ВЫКЛ"),
    ("Particles: All", "Частицы: Все"),
    ("Particles: Decreased", "Частицы: Меньше"),
    ("Particles: Minimal", "Частицы: Минимум"),
    ("Fullscreen: ON", "Полный экран: ВКЛ"),
    ("Fullscreen: OFF", "Полный экран: ВЫКЛ"),
    ("Exclusive Fullscreen: OFF", "Полноэкранный режим: ВЫКЛ"),
    ("VSync: ON", "Вертикальная синхронизация: ВКЛ"),
    ("VSync: OFF", "Вертикальная синхронизация: ВЫКЛ"),
    ("gui_scale_auto", "gui_scale_auto"),
    // Controls.
    ("Controls", "Управление"),
    ("Mouse Settings...", "Настройки мыши..."),
    ("Mouse Settings", "Настройки мыши"),
    ("Key Binds...", "Клавиши..."),
    ("Sneak: Hold", "Красться: Удержание"),
    ("Sneak: Toggle", "Красться: Переключение"),
    ("Sprint: Hold", "Бег: Удержание"),
    ("Sprint: Toggle", "Бег: Переключение"),
    ("Attack/Destroy: Hold", "Атака/Ломание: Удержание"),
    ("Attack/Destroy: Toggle", "Атака/Ломание: Переключение"),
    ("Use Item/Place Block: Hold", "Использование: Удержание"),
    ("Use Item/Place Block: Toggle", "Использование: Переключение"),
    ("Auto-Jump: ON", "Автопрыжок: ВКЛ"),
    ("Auto-Jump: OFF", "Автопрыжок: ВЫКЛ"),
    ("Sensitivity: *yawn*", "Чувствительность: *зевок*"),
    ("Sensitivity: HYPERSPEED!!!", "Чувствительность: ГИПЕРСКОРОСТЬ!!!"),
    ("Discrete Scrolling: OFF", "Дискретная прокрутка: ВЫКЛ"),
    ("Allow Cursor Changes: ON", "Смена курсора: ВКЛ"),
    ("Raw Input: ON", "Прямой ввод: ВКЛ"),
    // Sounds.
    ("Music & Sounds", "Музыка и звуки"),
    ("Music", "Музыка"),
    ("Sound", "Звук"),
    ("Master Volume", "Общая громкость"),
    // Pause menu.
    ("Return to Game", "Вернуться в игру"),
    ("Advancements", "Достижения"),
    ("Statistics", "Статистика"),
    ("Give Feedback", "Отзыв"),
    ("Report Bugs", "Сообщить об ошибке"),
    ("Options", "Настройки"),
    ("Save and Quit to Title", "Сохранить и выйти"),
    // Server list.
    ("Server List", "Список серверов"),
    ("Direct Connection", "Прямое подключение"),
    ("Add Server", "Добавить сервер"),
    ("Edit Server", "Изменить сервер"),
    ("Join Server", "Подключиться"),
    ("Refreshing...", "Обновление..."),
    ("Pinging...", "Проверка..."),
    ("Can't connect to server", "Не удаётся подключиться"),
    ("No connection", "Нет соединения"),
    // Account / sign-in.
    ("Sign in", "Войти"),
    ("Sign in with Microsoft", "Войти через Microsoft"),
    ("Sign Out", "Выйти"),
    ("Signed in as", "Вы вошли как"),
    ("Login failed", "Не удалось войти"),
    ("Play Offline", "Играть офлайн"),
    ("Offline", "Офлайн"),
    ("Player name (offline):", "Имя игрока (офлайн):"),
    ("Rename", "Переименовать"),
    ("WinModing needs a Minecraft account to play online.", "Для игры онлайн нужен аккаунт Minecraft."),
    ("Sign in with the Microsoft account that owns Minecraft, or play offline.", "Войдите в аккаунт Microsoft, владеющий Minecraft, или играйте офлайн."),
    ("Sign in with a Microsoft account to join online servers.", "Войдите через Microsoft, чтобы играть на серверах."),
    ("Contacting Microsoft...", "Связь с Microsoft..."),
    // Loading.
    ("Loading...", "Загрузка..."),
    ("Connecting...", "Подключение..."),
    ("Joining world...", "Вход в мир..."),
    ("Loading world...", "Загрузка мира..."),
    ("Generating world...", "Генерация мира..."),
    ("Saving world...", "Сохранение мира..."),
    ("Saving chunks", "Сохранение чанков"),
    ("Updating WinModing", "Обновление WinModing"),
    ("Downloading...", "Скачивание..."),
    ("Extracting...", "Распаковка..."),
    // Misc screens.
    ("Resource Packs", "Ресурспаки"),
    ("Chat Settings", "Настройки чата"),
    ("Font Settings", "Настройки шрифта"),
    ("Font Settings...", "Настройки шрифта..."),
    ("Credits & Attribution", "Авторы и благодарности"),
    ("Online", "Онлайн"),
    ("Skin Customization", "Настройка скина"),
    ("English", "English"),
    ("Русский", "Русский"),
];
