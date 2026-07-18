# 🤖 PosterBot — Анонимная предложка для Telegram

> 🇬🇧 **English version:** [README.md](README.md)

> Вы также можете попробовать моё готовое решение https://t.me/vsanonkabot

бот для приема анонимных предложений, написанный на Rust с использованием `teloxide` и `SQLite`. Бот позволяет пользователям отправлять идеи, текст, фото, видео, аудио и альбомы (медиагруппы) анонимно. Модераторы могут просматривать предложения, одобрять их (публикуя в канал), отклонять или блокировать пользователей.

## ✨ Возможности

- **Анонимные предложения**: Пользователи отправляют контент в личные сообщения бота, модераторы видят только контент.
- **Поддержка медиа**: Текст, фото, видео, аудио, голосовые, кружки (video notes), стикеры и **альбомы (media groups)**.
- **Модерация**: Удобная панель с Inline-кнопками (Одобрить / Отклонить / Бан / Указать причину).
- **Ответы на посты**: Пользователи могут отвечать на уже опубликованные посты через deeplink-кнопку.
- **Система банов**: Блокировка пользователей с генерацией уникального кода апелляции (BAN-ID) и команда `/pardon` для разблокировки.
- **Интернационализация (i18n)**: Поддержка английского и русского языков. Владелец может переключать язык командой `/lang <en|ru>`.
- **Надежное хранилище**: SQLite с включенным WAL-режимом. Дубликаты сообщений отбрасываются.


## ⚙️ Настройка

Создайте файл `.env` в корне проекта по примеру `.env.example`:

```env
TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
OWNER_ID=123456789
CHANNEL_ID=-1001234567890
OWNER_NAME=Ваше_Имя
BOT_USERNAME=your_bot_username
DB_PATH=data/bot.db
RUST_LOG=posterbot=info,teloxide=info
```

- `TELEGRAM_BOT_TOKEN`: Токен бота от [@BotFather](https://t.me/BotFather).
- `OWNER_ID`: Ваш числовой Telegram ID (можно узнать у [@userinfobot](https://t.me/userinfobot)).
- `CHANNEL_ID`: ID канала, куда будут публиковаться одобренные посты (начинается с `-100...`). Бот должен быть администратором канала.
- `OWNER_NAME`: Ваше имя пользователя (без @) или любое имя.
- `BOT_USERNAME`: Юзернейм бота (без @).
- `DB_PATH`: Путь к файлу базы данных (по умолчанию `data/bot.db`).

## 🚀 Развертывание (Деплой)

### Вариант 1: Docker Compose (Рекомендуется)

1. Убедитесь, что у вас установлены Docker и Docker Compose.
2. Создайте файл `.env` как описано выше.
3. Запустите проект:

```bash
docker compose up -d --build
```

Бот автоматически запустится в фоновом режиме. База данных будет храниться в папке `./data` на хост-машине, что предотвращает ее потерю при пересоздании контейнера.

**Просмотр логов:**
```bash
docker compose logs -f
```

### Вариант 2: Systemd (VPS / Bare Metal)

1. Установите Rust toolchain:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. Склонируйте репозиторий и скомпилируйте бинарник:
   ```bash
   cargo build --release
   ```

3. Создайте директорию для бота и скопируйте файлы:
   ```bash
   sudo mkdir -p /opt/posterbot/data
   sudo cp target/release/posterbot /opt/posterbot/
   sudo cp .env /opt/posterbot/
   sudo cp -r migrations /opt/posterbot/
   ```

4. Создайте пользователя и выставьте права:
   ```bash
   sudo useradd -r -s /bin/false -d /opt/posterbot posterbot
   sudo chown -R posterbot:posterbot /opt/posterbot
   sudo chmod 600 /opt/posterbot/.env
   ```

5. Создайте systemd-сервис `/etc/systemd/system/posterbot.service`:
   ```ini
   [Unit]
   Description=Telegram Anonymous Proposal Bot (Rust)
   After=network-online.target
   Wants=network-online.target

   [Service]
   Type=simple
   User=posterbot
   Group=posterbot
   WorkingDirectory=/opt/posterbot
   EnvironmentFile=/opt/posterbot/.env
   ExecStart=/opt/posterbot/posterbot
   Restart=always
   RestartSec=10
   StandardOutput=journal
   StandardError=journal
   SyslogIdentifier=posterbot
   NoNewPrivileges=true
   ProtectSystem=strict
   ProtectHome=true
   ReadWritePaths=/opt/posterbot/data
   PrivateTmp=true

   [Install]
   WantedBy=multi-user.target
   ```

6. Запустите и добавьте в автозагрузку:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable --now posterbot
   ```

**Просмотр логов:**
```bash
sudo journalctl -u posterbot -f
```

## 📋 Команды бота

### Пользовательские команды
- `/start` — Начать работу с ботом / открыть панель модератора (если есть права).
- `/start reply_<ID>` — (Автоматически) Ответить на конкретный пост в канале.
- `/reply <ID>` — Ответить на пост по его числовому ID.

### Команды модератора
- `/proposals` — Открыть очередь немодерированных предложений.

### Команды владельца (Owner)
- `/addadmin <ID>` — Добавить модератора по Telegram ID.
- `/removeadmin <ID>` — Удалить модератора.
- `/admins` — Список модераторов.
- `/banned` — Список активных блокировок.
- `/pardon <BAN-ID>` — Снять блокировку с пользователя по коду апелляции.
- `/lang <en|ru>` — Изменить язык интерфейса бота.

## 🧪 Разработка и тесты

Проект включает модульные тесты для проверки логики базы данных (предотвращение дубликатов, группировка медиа, состояния пользователей). Для запуска тестов выполните:

```bash
cargo test
```

## 📄 Лицензия

MIT License. Свободно используйте и модифицируйте код.
