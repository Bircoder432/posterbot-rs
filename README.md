# 🤖 PosterBot — Anonymous Telegram Suggestion Bot

> 🇷🇺 **Русская версия:** [README_RU.md](README_RU.md)
> You also can try my ready-made solution https://t.me/vsanonkabot 


PosterBot is a Telegram bot for receiving anonymous suggestions, written in Rust using `teloxide` and `SQLite`. It allows users to anonymously submit ideas, text, photos, videos, audio, and media albums (media groups). Moderators can review submissions, approve them for publication in a Telegram channel, reject them, or ban users.

## ✨ Features

* **Anonymous submissions**: Users send content via private messages to the bot, while moderators only see the submitted content.
* **Media support**: Text, photos, videos, audio, voice messages, video notes, stickers, and **media albums (media groups)**.
* **Moderation panel**: Convenient inline keyboard with **Approve / Reject / Ban / Specify reason** actions.
* **Post replies**: Users can reply to published posts using deep-link buttons.
* **Ban system**: Ban users with automatically generated appeal codes (BAN-ID) and unban them using the `/pardon` command.
* **Internationalization (i18n)**: Supports English and Russian. The owner can switch the bot language using `/lang <en|ru>`.
* **Reliable storage**: SQLite with WAL mode enabled. Duplicate messages are automatically discarded.

## ⚙️ Configuration

Create a `.env` file in the project root based on `.env.example`:

```env
TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
OWNER_ID=123456789
CHANNEL_ID=-1001234567890
OWNER_NAME=Your_Name
BOT_USERNAME=your_bot_username
DB_PATH=data/bot.db
RUST_LOG=posterbot=info,teloxide=info
```

* `TELEGRAM_BOT_TOKEN`: Your bot token from [@BotFather](https://t.me/BotFather).
* `OWNER_ID`: Your numeric Telegram ID (you can get it from [@userinfobot](https://t.me/userinfobot)).
* `CHANNEL_ID`: The ID of the Telegram channel where approved posts will be published (starts with `-100...`). The bot must be an administrator of the channel.
* `OWNER_NAME`: Your username (without `@`) or any display name.
* `BOT_USERNAME`: Your bot's username (without `@`).
* `DB_PATH`: Path to the SQLite database file (default: `data/bot.db`).

## 🚀 Deployment

### Option 1: Docker Compose (Recommended)

1. Make sure Docker and Docker Compose are installed.
2. Create the `.env` file as described above.
3. Start the project:

```bash
docker compose up -d --build
```

The bot will start in the background automatically. The database will be stored in the `./data` directory on the host machine, preventing data loss when recreating the container.

**View logs:**

```bash
docker compose logs -f
```

### Option 2: Systemd (VPS / Bare Metal)

1. Install the Rust toolchain:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. Clone the repository and build the binary:

   ```bash
   cargo build --release
   ```

3. Create a directory for the bot and copy the required files:

   ```bash
   sudo mkdir -p /opt/posterbot/data
   sudo cp target/release/posterbot /opt/posterbot/
   sudo cp .env /opt/posterbot/
   sudo cp -r migrations /opt/posterbot/
   ```

4. Create a dedicated user and set permissions:

   ```bash
   sudo useradd -r -s /bin/false -d /opt/posterbot posterbot
   sudo chown -R posterbot:posterbot /opt/posterbot
   sudo chmod 600 /opt/posterbot/.env
   ```

5. Create the systemd service `/etc/systemd/system/posterbot.service`:

   ```ini
   [Unit]
   Description=Telegram Anonymous Suggestion Bot (Rust)
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

6. Enable and start the service:

   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable --now posterbot
   ```

**View logs:**

```bash
sudo journalctl -u posterbot -f
```

## 📋 Bot Commands

### User Commands

* `/start` — Start using the bot or open the moderator panel (if authorized).
* `/start reply_<ID>` — *(Automatically used)* Reply to a specific channel post.
* `/reply <ID>` — Reply to a published post by its numeric ID.

### Moderator Commands

* `/proposals` — Open the moderation queue of pending submissions.

### Owner Commands

* `/addadmin <ID>` — Add a moderator by Telegram ID.
* `/removeadmin <ID>` — Remove a moderator.
* `/admins` — List all moderators.
* `/banned` — Show all active bans.
* `/pardon <BAN-ID>` — Unban a user using their appeal code.
* `/lang <en|ru>` — Change the bot interface language.

## 🧪 Development & Testing

The project includes unit tests covering database logic such as duplicate prevention, media grouping, and user state management.

Run the test suite with:

```bash
cargo test
```

## 📄 License

Licensed under the MIT License. Feel free to use, modify, and distribute the project.
