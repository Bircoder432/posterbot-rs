#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    En,
    Ru,
}

impl Locale {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" => Some(Locale::En),
            "ru" => Some(Locale::Ru),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Ru => "ru",
        }
    }
}

pub struct L10n;

impl L10n {
    pub fn welcome(l: Locale) -> &'static str {
        match l {
            Locale::En => {
                "🤖 Welcome to the anonymous proposal bot!\n\nJust send your proposal, idea, or message here, and it will be anonymously reviewed by moderators.\n\nYour identity will be hidden - moderators will only see the content of your message.\n\n❓ What you can send:\n• Text proposals\n• Photos\n• Documents\n• Videos\n• Video notes\n• Audio and voice messages\n• Stickers\n\nYour proposal will be reviewed shortly!"
            }
            Locale::Ru => {
                "🤖 Добро пожаловать в анонимную предложку!\n\nПросто отправьте сюда ваше предложение, идею или сообщение, и оно будет анонимно рассмотрено модераторами.\n\nВаша личность будет скрыта - модераторы увидят только содержание вашего сообщения.\n\n❓ Что можно отправлять:\n• Текстовые предложения\n• Фотографии\n• Документы\n• Видео\n• Кружочки (видеосообщения)\n• Аудио и голосовые сообщения\n• Стикеры\n\nВаше предложение будет рассмотрено в ближайшее время!"
            }
        }
    }

    pub fn only_owner_add_admins(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Only the owner can add admins.",
            Locale::Ru => "❌ Только владелец бота может добавлять администраторов.",
        }
    }

    pub fn add_admin_usage(l: Locale) -> &'static str {
        match l {
            Locale::En => "📝 Usage: /addadmin <user_ID>\n\nExample: /addadmin 123456789",
            Locale::Ru => {
                "📝 Использование: /addadmin <ID_пользователя>\n\nПример: /addadmin 123456789"
            }
        }
    }

    pub fn already_owner(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ You are already the owner.",
            Locale::Ru => "❌ Вы уже являетесь владельцем бота.",
        }
    }

    pub fn admin_already_exists(l: Locale, id: i64) -> String {
        match l {
            Locale::En => format!("❌ User {id} is already an admin."),
            Locale::Ru => format!("❌ Пользователь {id} уже является администратором."),
        }
    }

    pub fn admin_added(l: Locale, name: &str) -> String {
        match l {
            Locale::En => format!("✅ User {name} added as admin!"),
            Locale::Ru => format!("✅ Пользователь {name} добавлен как администратор!"),
        }
    }

    pub fn admin_added_notification(l: Locale) -> &'static str {
        match l {
            Locale::En => {
                "🎉 You have been added as a moderator!\n\nUse /start to access the moderation panel."
            }
            Locale::Ru => {
                "🎉 Вы были добавлены как модератор бота-предложки!\n\nИспользуйте команду /start для доступа к панели модерации."
            }
        }
    }

    pub fn only_owner_remove_admins(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Only the owner can remove admins.",
            Locale::Ru => "❌ Только владелец может удалять администраторов.",
        }
    }

    pub fn remove_admin_usage(l: Locale) -> &'static str {
        match l {
            Locale::En => "📝 Usage: /removeadmin <user_ID>\n\nExample: /removeadmin 123456789",
            Locale::Ru => {
                "📝 Использование: /removeadmin <ID_пользователя>\n\nПример: /removeadmin 123456789"
            }
        }
    }

    pub fn cannot_remove_owner(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Cannot remove the owner.",
            Locale::Ru => "❌ Нельзя удалить владельца бота.",
        }
    }

    pub fn admin_not_found(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ User is not an admin.",
            Locale::Ru => "❌ Пользователь не является администратором.",
        }
    }

    pub fn admin_removed(l: Locale, id: i64) -> String {
        match l {
            Locale::En => format!("✅ Admin {id} removed."),
            Locale::Ru => format!("✅ Администратор {id} удалён."),
        }
    }

    pub fn admin_removed_notification(l: Locale) -> &'static str {
        match l {
            Locale::En => "⚠️ You are no longer a moderator.",
            Locale::Ru => "⚠️ Вы больше не являетесь модератором бота.",
        }
    }

    pub fn only_owner_list_admins(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Only the owner can list admins.",
            Locale::Ru => "❌ Только владелец может просматривать список администраторов.",
        }
    }

    pub fn no_admins(l: Locale) -> &'static str {
        match l {
            Locale::En => "📋 No moderators.",
            Locale::Ru => "📋 Список модераторов пуст.",
        }
    }

    pub fn admins_list_header(l: Locale, owner_id: i64) -> String {
        match l {
            Locale::En => format!("📋 Moderators:\n\n👑 Owner: ID {owner_id}\n"),
            Locale::Ru => format!("📋 Список модераторов:\n\n👑 Владелец: ID {owner_id}\n"),
        }
    }

    pub fn no_access(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ No access.",
            Locale::Ru => "❌ У вас нет доступа к этой функции.",
        }
    }

    pub fn no_active_bans(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ No active bans.",
            Locale::Ru => "✅ Список банов пуст.",
        }
    }

    pub fn active_bans_header(l: Locale) -> &'static str {
        match l {
            Locale::En => "🚫 Active bans:\n\n",
            Locale::Ru => "🚫 Активные блокировки:\n\n",
        }
    }

    pub fn ban_record_entry(l: Locale, i: usize, ban_id: &str, reason: &str, date: &str) -> String {
        match l {
            Locale::En => format!("{i}. {ban_id}\n   Reason: {reason}\n   Date: {date}\n"),
            Locale::Ru => format!("{i}. {ban_id}\n   Причина: {reason}\n   Дата: {date}\n"),
        }
    }

    pub fn pardon_usage(l: Locale) -> &'static str {
        match l {
            Locale::En => "📝 Usage: /pardon <BAN-ID>",
            Locale::Ru => "📝 Использование: /pardon <BAN-ID>",
        }
    }

    pub fn ban_not_found(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Ban not found or already pardoned.",
            Locale::Ru => "❌ Бан не найден или уже снят.",
        }
    }

    pub fn access_restored(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Your access has been restored.",
            Locale::Ru => "✅ Ваш доступ восстановлен.",
        }
    }

    pub fn ban_deactivated(l: Locale, ban_id: &str) -> String {
        match l {
            Locale::En => format!("✅ Ban {ban_id} deactivated."),
            Locale::Ru => format!("✅ Бан {ban_id} деактивирован."),
        }
    }

    pub fn no_new_proposals(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ No new proposals.",
            Locale::Ru => "✅ Нет новых предложений.",
        }
    }

    pub fn proposal_items_count(l: Locale, count: usize) -> String {
        match l {
            Locale::En => format!("📨 Proposal ({count} item(s)):",),
            Locale::Ru => format!("📨 Предложение ({count} элем.):"),
        }
    }

    pub fn failed_display_media(l: Locale, e: &str) -> String {
        match l {
            Locale::En => format!("❌ Failed to display media: {e}"),
            Locale::Ru => format!("❌ Ошибка отображения медиа: {e}"),
        }
    }

    pub fn proposal_action_header(l: Locale, id: i64, date: &str) -> String {
        match l {
            Locale::En => format!("📨 Proposal #{id}\n⏰ {date}\n\nChoose action:"),
            Locale::Ru => format!("📨 Предложение #{id}\n⏰ {date}\n\nВыберите действие:"),
        }
    }

    pub fn approve_btn(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ APPROVE",
            Locale::Ru => "✅ ОДОБРИТЬ",
        }
    }

    pub fn reject_btn(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ REJECT",
            Locale::Ru => "❌ ОТКЛОНИТЬ",
        }
    }

    pub fn published(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Published!",
            Locale::Ru => "✅ Опубликовано!",
        }
    }

    pub fn published_no_id(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Published but failed to retrieve message ID",
            Locale::Ru => "❌ Опубликовано, но не удалось получить ID сообщения",
        }
    }

    pub fn failed_publish(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Failed to publish",
            Locale::Ru => "❌ Ошибка публикации",
        }
    }

    pub fn rejected(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Rejected!",
            Locale::Ru => "✅ Отклонено!",
        }
    }

    pub fn reason_btn(l: Locale) -> &'static str {
        match l {
            Locale::En => "Reason",
            Locale::Ru => "Причина",
        }
    }

    pub fn next_btn(l: Locale) -> &'static str {
        match l {
            Locale::En => "Next",
            Locale::Ru => "Далее",
        }
    }

    pub fn ban_btn(l: Locale) -> &'static str {
        match l {
            Locale::En => "Ban",
            Locale::Ru => "Бан",
        }
    }

    pub fn choose_action(l: Locale) -> &'static str {
        match l {
            Locale::En => "Choose action:",
            Locale::Ru => "Выберите действие:",
        }
    }

    pub fn enter_rejection_reason(l: Locale) -> &'static str {
        match l {
            Locale::En => "Enter rejection reason:",
            Locale::Ru => "Введите причину отказа:",
        }
    }

    pub fn enter_reason_callback(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Enter reason",
            Locale::Ru => "✅ Введите причину",
        }
    }

    pub fn enter_ban_reason(l: Locale) -> &'static str {
        match l {
            Locale::En => "Enter ban reason:",
            Locale::Ru => "Введите причину блокировки:",
        }
    }

    pub fn send_reply_to_post(l: Locale) -> &'static str {
        match l {
            Locale::En => "✍️ Send your reply to the post.",
            Locale::Ru => "✍️ Отправьте ваш ответ на пост.",
        }
    }

    pub fn invalid_reply_link(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Invalid reply link.",
            Locale::Ru => "❌ Неверная ссылка для ответа.",
        }
    }

    pub fn user_banned(l: Locale) -> &'static str {
        match l {
            Locale::En => "🚫 You are banned.",
            Locale::Ru => "🚫 Вы заблокированы.",
        }
    }

    pub fn owner_panel(l: Locale) -> &'static str {
        match l {
            Locale::En => {
                "👑 Owner Panel\n\nCommands:\n/addadmin <ID>\n/removeadmin <ID>\n/admins\n/banned\n/proposals\n/pardon <BAN-ID>\n/lang <en|ru>"
            }
            Locale::Ru => {
                "👑 Панель владельца\n\nКоманды:\n/addadmin <ID>\n/removeadmin <ID>\n/admins\n/banned\n/proposals\n/pardon <BAN-ID>\n/lang <en|ru>"
            }
        }
    }

    pub fn mod_panel(l: Locale) -> &'static str {
        match l {
            Locale::En => "🛠️ Moderator Panel\n\nCommands:\n/proposals",
            Locale::Ru => "🛠️ Панель модератора\n\nКоманды:\n/proposals",
        }
    }

    pub fn reply_usage(l: Locale) -> &'static str {
        match l {
            Locale::En => "📝 Usage: /reply <post_ID>",
            Locale::Ru => "📝 Использование: /reply <ID_поста>",
        }
    }

    pub fn post_not_found(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Post not found.",
            Locale::Ru => "❌ Пост не найден.",
        }
    }

    pub fn proposal_accepted(l: Locale) -> &'static str {
        match l {
            Locale::En => {
                "✅ Your proposal has been accepted! It will be reviewed by moderators anonymously."
            }
            Locale::Ru => {
                "✅ Ваше предложение принято! Оно будет рассмотрено модераторами анонимно."
            }
        }
    }

    pub fn reply_accepted(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Your reply has been accepted!",
            Locale::Ru => "✅ Ваш ответ принят!",
        }
    }

    pub fn error_sending_reply(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Error sending reply.",
            Locale::Ru => "❌ Ошибка при отправке ответа.",
        }
    }

    pub fn rejected_reason(l: Locale, reason: &str) -> String {
        match l {
            Locale::En => format!("Your message was rejected. Reason: {reason}"),
            Locale::Ru => format!("Ваше сообщение отклонено по причине: {reason}"),
        }
    }

    pub fn reason_sent(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Reason sent.",
            Locale::Ru => "✅ Причина отправлена.",
        }
    }

    pub fn user_banned_appeal(l: Locale, ban_id: &str) -> String {
        match l {
            Locale::En => format!("🚫 You are banned. Appeal code: {ban_id}"),
            Locale::Ru => format!("🚫 Вы заблокированы. Код обращения: {ban_id}"),
        }
    }

    pub fn user_banned_success(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ User banned.",
            Locale::Ru => "✅ Пользователь заблокирован.",
        }
    }

    pub fn error_banning_user(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Error banning user.",
            Locale::Ru => "❌ Ошибка при блокировке.",
        }
    }

    pub fn new_proposal_notif(l: Locale, id: &str, text: &str, media_type: &str) -> String {
        match l {
            Locale::En => format!(
                "📨 New proposal!\n\nID: {id}\n💬 {text}\n📁 Type: {media_type}\n\n/proposals"
            ),
            Locale::Ru => format!(
                "📨 Новое предложение!\n\nID: {id}\n💬 {text}\n📁 Тип: {media_type}\n\n/proposals"
            ),
        }
    }

    pub fn text_proposal(l: Locale, caption: &str) -> String {
        match l {
            Locale::En => format!("💬 Text proposal:\n{caption}"),
            Locale::Ru => format!("💬 Текст предложения:\n{caption}"),
        }
    }

    pub fn media_proposal_text(l: Locale, media_type: &str, caption: &str) -> String {
        match l {
            Locale::En => format!("💬 {media_type}\n{caption}"),
            Locale::Ru => format!("💬 {media_type}\n{caption}"),
        }
    }

    pub fn photo_text(l: Locale) -> &'static str {
        match l {
            Locale::En => "🖼️ Photo",
            Locale::Ru => "🖼️ Фото",
        }
    }

    pub fn document_text(l: Locale, name: &str) -> String {
        match l {
            Locale::En => format!("📄 Document: {name}"),
            Locale::Ru => format!("📄 Документ: {name}"),
        }
    }

    pub fn video_text(l: Locale) -> &'static str {
        match l {
            Locale::En => "🎥 Video",
            Locale::Ru => "🎥 Видео",
        }
    }

    pub fn video_note_text(l: Locale) -> &'static str {
        match l {
            Locale::En => "📹 Video note",
            Locale::Ru => "📹 Кружочек (видеосообщение)",
        }
    }

    pub fn audio_text(l: Locale, title: &str) -> String {
        match l {
            Locale::En => format!("🎵 {title}"),
            Locale::Ru => format!("🎵 {title}"),
        }
    }

    pub fn voice_text(l: Locale) -> &'static str {
        match l {
            Locale::En => "🎤 Voice message",
            Locale::Ru => "🎤 Голосовое сообщение",
        }
    }

    pub fn sticker_text(l: Locale) -> &'static str {
        match l {
            Locale::En => "😊 Sticker",
            Locale::Ru => "😊 Стикер",
        }
    }

    pub fn media_content_text(l: Locale) -> &'static str {
        match l {
            Locale::En => "📦 Media content",
            Locale::Ru => "📦 Медиа-контент",
        }
    }

    pub fn reply_quote(l: Locale, quoted: &str) -> String {
        match l {
            Locale::En => format!("💬 Reply:\n\n{quoted}"),
            Locale::Ru => format!("💬 Ответ:\n\n{quoted}"),
        }
    }

    pub fn lang_usage(l: Locale) -> &'static str {
        match l {
            Locale::En => "📝 Usage: /lang <en|ru>",
            Locale::Ru => "📝 Использование: /lang <en|ru>",
        }
    }

    pub fn lang_invalid(l: Locale) -> &'static str {
        match l {
            Locale::En => "❌ Invalid language. Use 'en' or 'ru'.",
            Locale::Ru => "❌ Неверный язык. Используйте 'en' или 'ru'.",
        }
    }

    pub fn lang_updated(l: Locale) -> &'static str {
        match l {
            Locale::En => "✅ Language updated to English.",
            Locale::Ru => "✅ Язык изменен на Русский.",
        }
    }
}
