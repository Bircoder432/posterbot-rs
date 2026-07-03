use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{
    InputFile, InputMedia, InputMediaAudio, InputMediaPhoto, InputMediaVideo, LinkPreviewOptions,
    Message as TgMessage, MessageId, ParseMode, ReplyParameters,
};

use crate::db::models::{Message, Proposal};
use crate::locales::{L10n, Locale};

pub fn extract_media_info(msg: &TgMessage, _lang: Locale) -> (String, String) {
    if let Some(photo) = msg.photo().and_then(|p| p.last()) {
        return ("photo".into(), photo.file.id.clone());
    }
    if let Some(doc) = msg.document() {
        return ("document".into(), doc.file.id.clone());
    }
    if let Some(video) = msg.video() {
        return ("video".into(), video.file.id.clone());
    }
    if let Some(audio) = msg.audio() {
        return ("audio".into(), audio.file.id.clone());
    }
    if let Some(voice) = msg.voice() {
        return ("voice".into(), voice.file.id.clone());
    }
    if let Some(sticker) = msg.sticker() {
        return ("sticker".into(), sticker.file.id.clone());
    }
    if let Some(vn) = msg.video_note() {
        return ("video_note".into(), vn.file.id.clone());
    }
    ("text".into(), String::new())
}

pub fn extract_message_text(msg: &TgMessage, lang: Locale) -> String {
    if let Some(text) = msg.text() {
        return text.to_string();
    }
    if let Some(caption) = msg.caption() {
        return caption.to_string();
    }
    if msg.media_group_id().is_some() {
        return String::new();
    }
    if msg.photo().map_or(false, |p| !p.is_empty()) {
        return L10n::photo_text(lang).into();
    }
    if let Some(doc) = msg.document() {
        return L10n::document_text(lang, doc.file_name.as_deref().unwrap_or("unknown"));
    }
    if msg.video().is_some() {
        return L10n::video_text(lang).into();
    }
    if msg.video_note().is_some() {
        return L10n::video_note_text(lang).into();
    }
    if let Some(audio) = msg.audio() {
        return L10n::audio_text(lang, audio.title.as_deref().unwrap_or("Audio"));
    }
    if msg.voice().is_some() {
        return L10n::voice_text(lang).into();
    }
    if msg.sticker().is_some() {
        return L10n::sticker_text(lang).into();
    }
    L10n::media_content_text(lang).into()
}

pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn input_file(file_id: &str) -> InputFile {
    InputFile::file_id(file_id.to_string())
}

fn disabled_preview_options() -> LinkPreviewOptions {
    LinkPreviewOptions {
        is_disabled: true,
        url: None,
        prefer_small_media: false,
        prefer_large_media: false,
        show_above_text: false,
    }
}

pub async fn send_for_moderation(
    bot: &Bot,
    chat_id: ChatId,
    proposal: &Proposal,
    lang: Locale,
) -> Result<()> {
    if proposal.messages.len() == 1 {
        send_single_for_moderation(bot, chat_id, proposal.first(), lang).await
    } else {
        send_group_for_moderation(bot, chat_id, &proposal.messages, lang).await
    }
}

async fn send_single_for_moderation(
    bot: &Bot,
    chat_id: ChatId,
    msg: &Message,
    lang: Locale,
) -> Result<()> {
    let media_type = msg.media_type.as_str();
    let file_id = msg.media_file_id.as_str();
    let caption = &msg.message_text;

    if media_type == "text" || file_id.is_empty() {
        bot.send_message(chat_id, L10n::text_proposal(lang, caption))
            .await?;
        return Ok(());
    }

    let file = input_file(file_id);
    match media_type {
        "photo" => bot.send_photo(chat_id, file).caption(caption).await?,
        "document" => bot.send_document(chat_id, file).caption(caption).await?,
        "video" => bot.send_video(chat_id, file).caption(caption).await?,
        "audio" => bot.send_audio(chat_id, file).caption(caption).await?,
        "voice" => bot.send_voice(chat_id, file).caption(caption).await?,
        "sticker" => bot.send_sticker(chat_id, file).await?,
        "video_note" => bot.send_video_note(chat_id, file).await?,
        _ => {
            bot.send_message(
                chat_id,
                L10n::media_proposal_text(lang, media_type, caption),
            )
            .await?
        }
    };
    Ok(())
}

async fn send_group_for_moderation(
    bot: &Bot,
    chat_id: ChatId,
    messages: &[Message],
    lang: Locale,
) -> Result<()> {
    let all_media = messages
        .iter()
        .all(|m| matches!(m.media_type.as_str(), "photo" | "video" | "audio"));

    if all_media {
        let media: Vec<InputMedia> = messages
            .iter()
            .enumerate()
            .map(|(i, m)| build_input_media(m, i == 0, false))
            .collect();
        bot.send_media_group(chat_id, media).await?;
    } else {
        for m in messages {
            send_single_for_moderation(bot, chat_id, m, lang).await?;
        }
    }
    Ok(())
}

fn build_input_media(msg: &Message, is_first: bool, with_html: bool) -> InputMedia {
    let file = input_file(&msg.media_file_id);
    let caption = if is_first && !msg.message_text.is_empty() {
        Some(msg.message_text.clone())
    } else {
        None
    };

    let mut builder = match msg.media_type.as_str() {
        "video" => InputMediaBuilder::Video(InputMediaVideo::new(file)),
        "audio" => InputMediaBuilder::Audio(InputMediaAudio::new(file)),
        _ => InputMediaBuilder::Photo(InputMediaPhoto::new(file)),
    };

    if let Some(cap) = caption {
        builder = builder.caption(cap);
        if with_html {
            builder = builder.parse_mode(ParseMode::Html);
        }
    }

    builder.build()
}

enum InputMediaBuilder {
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    Audio(InputMediaAudio),
}

impl InputMediaBuilder {
    fn caption(self, caption: String) -> Self {
        match self {
            Self::Photo(b) => Self::Photo(b.caption(caption)),
            Self::Video(b) => Self::Video(b.caption(caption)),
            Self::Audio(b) => Self::Audio(b.caption(caption)),
        }
    }

    fn parse_mode(self, mode: ParseMode) -> Self {
        match self {
            Self::Photo(b) => Self::Photo(b.parse_mode(mode)),
            Self::Video(b) => Self::Video(b.parse_mode(mode)),
            Self::Audio(b) => Self::Audio(b.parse_mode(mode)),
        }
    }

    fn build(self) -> InputMedia {
        match self {
            Self::Photo(b) => InputMedia::Photo(b),
            Self::Video(b) => InputMedia::Video(b),
            Self::Audio(b) => InputMedia::Audio(b),
        }
    }
}

pub async fn publish(
    bot: &Bot,
    channel_id: i64,
    proposal: &Proposal,
    bot_username: &str,
    reply_to: Option<i32>,
    lang: Locale,
) -> Result<Option<i32>> {
    let first = proposal.first();
    let escaped_text = escape_html(&first.message_text);
    let quoted = format!("<blockquote>{escaped_text}</blockquote>");

    let is_reply = first.parent_message_id.is_some();
    let base = if is_reply {
        L10n::reply_quote(lang, &quoted)
    } else {
        quoted
    };

    let reply_link = format!(
        "\n\n<a href=\"https://t.me/{bot_username}?start=reply_{}\">💬 Reply</a>",
        first.id
    );
    let caption = format!("{base}{reply_link}");

    if proposal.messages.len() == 1 {
        publish_single(bot, channel_id, first, &caption, reply_to).await
    } else {
        publish_group(bot, channel_id, &proposal.messages, &caption).await
    }
}

async fn publish_single(
    bot: &Bot,
    channel_id: i64,
    msg: &Message,
    caption: &str,
    reply_to: Option<i32>,
) -> Result<Option<i32>> {
    let chat = ChatId(channel_id);
    let media_type = msg.media_type.as_str();
    let file_id = msg.media_file_id.as_str();

    let sent = if media_type == "text" || file_id.is_empty() {
        let mut req = bot
            .send_message(chat, caption)
            .parse_mode(ParseMode::Html)
            .link_preview_options(disabled_preview_options());
        if let Some(id) = reply_to {
            req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
        }
        req.await?
    } else {
        let file = input_file(file_id);
        match media_type {
            "photo" => {
                let mut req = bot
                    .send_photo(chat, file)
                    .caption(caption)
                    .parse_mode(ParseMode::Html);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            "document" => {
                let mut req = bot
                    .send_document(chat, file)
                    .caption(caption)
                    .parse_mode(ParseMode::Html);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            "video" => {
                let mut req = bot
                    .send_video(chat, file)
                    .caption(caption)
                    .parse_mode(ParseMode::Html);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            "audio" => {
                let mut req = bot
                    .send_audio(chat, file)
                    .caption(caption)
                    .parse_mode(ParseMode::Html);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            "voice" => {
                let mut req = bot
                    .send_voice(chat, file)
                    .caption(caption)
                    .parse_mode(ParseMode::Html);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            "sticker" => {
                let mut req = bot.send_sticker(chat, file);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            "video_note" => {
                let mut req = bot.send_video_note(chat, file);
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
            _ => {
                let mut req = bot
                    .send_message(chat, caption)
                    .parse_mode(ParseMode::Html)
                    .link_preview_options(disabled_preview_options());
                if let Some(id) = reply_to {
                    req = req.reply_parameters(ReplyParameters::new(MessageId(id)));
                }
                req.await?
            }
        }
    };

    Ok(Some(sent.id.0))
}

async fn publish_group(
    bot: &Bot,
    channel_id: i64,
    messages: &[Message],
    caption: &str,
) -> Result<Option<i32>> {
    let media: Vec<InputMedia> = messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            if i == 0 {
                let file = input_file(&m.media_file_id);
                match m.media_type.as_str() {
                    "video" => InputMedia::Video(
                        InputMediaVideo::new(file)
                            .caption(caption)
                            .parse_mode(ParseMode::Html),
                    ),
                    "audio" => InputMedia::Audio(
                        InputMediaAudio::new(file)
                            .caption(caption)
                            .parse_mode(ParseMode::Html),
                    ),
                    _ => InputMedia::Photo(
                        InputMediaPhoto::new(file)
                            .caption(caption)
                            .parse_mode(ParseMode::Html),
                    ),
                }
            } else {
                build_input_media(m, false, false)
            }
        })
        .collect();

    let sent_messages = bot.send_media_group(ChatId(channel_id), media).await?;
    Ok(sent_messages.first().map(|m| m.id.0))
}
