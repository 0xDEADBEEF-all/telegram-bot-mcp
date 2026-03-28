use crate::bot_api::BotApiClient;
use serde_json::Value;

/// List all available Telegram Bot API methods with descriptions.
pub fn list_methods() {
    let methods: &[(&str, &str)] = &[
        // Bot info
        ("getMe", "Get basic information about the bot"),
        ("logOut", "Log out from the cloud Bot API server"),
        ("close", "Close the bot instance before moving it"),
        ("setMyCommands", "Set the bot's command list"),
        ("deleteMyCommands", "Delete the bot's command list"),
        ("getMyCommands", "Get the bot's command list"),
        ("setMyName", "Set the bot's name"),
        ("getMyName", "Get the bot's name"),
        ("setMyDescription", "Set the bot's description"),
        ("getMyDescription", "Get the bot's description"),
        ("setMyShortDescription", "Set the bot's short description"),
        ("getMyShortDescription", "Get the bot's short description"),
        (
            "setChatMenuButton",
            "Set the bot's menu button in a private chat",
        ),
        ("getChatMenuButton", "Get the bot's menu button"),
        (
            "setMyDefaultAdministratorRights",
            "Set default admin rights for the bot",
        ),
        (
            "getMyDefaultAdministratorRights",
            "Get default admin rights of the bot",
        ),
        // Updates
        ("getUpdates", "Get incoming updates via long polling"),
        ("setWebhook", "Set a webhook URL for receiving updates"),
        ("deleteWebhook", "Remove the webhook integration"),
        ("getWebhookInfo", "Get current webhook status"),
        // Messages
        ("sendMessage", "Send a text message to a chat"),
        (
            "forwardMessage",
            "Forward a message from one chat to another",
        ),
        ("forwardMessages", "Forward multiple messages at once"),
        (
            "copyMessage",
            "Copy a message without 'Forwarded from' header",
        ),
        ("copyMessages", "Copy multiple messages at once"),
        (
            "sendChatAction",
            "Send a chat action (typing, uploading, etc.)",
        ),
        ("setMessageReaction", "Set a reaction on a message"),
        ("pinChatMessage", "Pin a message in a chat"),
        ("unpinChatMessage", "Unpin a message in a chat"),
        ("unpinAllChatMessages", "Unpin all pinned messages"),
        // Editing
        ("editMessageText", "Edit text of a message"),
        ("editMessageCaption", "Edit caption of a message"),
        ("editMessageMedia", "Edit media content of a message"),
        ("editMessageLiveLocation", "Edit a live location message"),
        ("stopMessageLiveLocation", "Stop updating a live location"),
        (
            "editMessageReplyMarkup",
            "Edit the reply markup of a message",
        ),
        ("stopPoll", "Stop a poll sent by the bot"),
        ("deleteMessage", "Delete a message"),
        ("deleteMessages", "Delete multiple messages at once"),
        // Media
        ("sendPhoto", "Send a photo"),
        ("sendAudio", "Send an audio file"),
        ("sendDocument", "Send a general file/document"),
        ("sendVideo", "Send a video file"),
        ("sendAnimation", "Send an animation (GIF)"),
        ("sendVoice", "Send a voice message"),
        ("sendVideoNote", "Send a video note (rounded square video)"),
        ("sendPaidMedia", "Send paid media (Stars)"),
        ("sendMediaGroup", "Send a group of photos/videos as album"),
        ("sendLocation", "Send a location point on the map"),
        ("sendVenue", "Send a venue"),
        ("sendContact", "Send a phone contact"),
        ("sendPoll", "Send a native poll"),
        ("sendDice", "Send an animated emoji dice"),
        // Chat management
        ("getChat", "Get up-to-date information about a chat"),
        (
            "getChatAdministrators",
            "Get list of administrators in a chat",
        ),
        ("getChatMemberCount", "Get the number of members in a chat"),
        ("getChatMember", "Get information about a member"),
        ("banChatMember", "Ban a user in a group/supergroup/channel"),
        ("unbanChatMember", "Unban a previously banned user"),
        ("restrictChatMember", "Restrict a user in a supergroup"),
        ("promoteChatMember", "Promote or demote a user"),
        (
            "setChatAdministratorCustomTitle",
            "Set a custom title for an admin",
        ),
        ("banChatSenderChat", "Ban a channel chat in a supergroup"),
        ("unbanChatSenderChat", "Unban a previously banned channel"),
        ("setChatPermissions", "Set default chat permissions"),
        ("exportChatInviteLink", "Generate a new primary invite link"),
        ("createChatInviteLink", "Create an additional invite link"),
        ("editChatInviteLink", "Edit an existing invite link"),
        ("revokeChatInviteLink", "Revoke an invite link"),
        (
            "createChatSubscriptionInviteLink",
            "Create a subscription invite link",
        ),
        (
            "editChatSubscriptionInviteLink",
            "Edit a subscription invite link",
        ),
        ("approveChatJoinRequest", "Approve a chat join request"),
        ("declineChatJoinRequest", "Decline a chat join request"),
        ("setChatPhoto", "Set a new chat photo"),
        ("deleteChatPhoto", "Delete a chat photo"),
        ("setChatTitle", "Set a new chat title"),
        ("setChatDescription", "Set the chat description"),
        ("leaveChat", "Leave a group/supergroup/channel"),
        ("setChatStickerSet", "Set a group sticker set"),
        ("deleteChatStickerSet", "Delete a group sticker set"),
        // Forum topics
        (
            "getForumTopicIconStickers",
            "Get custom emoji stickers for forum topic icons",
        ),
        ("createForumTopic", "Create a topic in a forum supergroup"),
        ("editForumTopic", "Edit name and icon of a forum topic"),
        ("closeForumTopic", "Close a forum topic"),
        ("reopenForumTopic", "Reopen a closed forum topic"),
        ("deleteForumTopic", "Delete a forum topic and all messages"),
        (
            "unpinAllForumTopicMessages",
            "Clear pinned messages in a topic",
        ),
        ("editGeneralForumTopic", "Edit the 'General' forum topic"),
        ("closeGeneralForumTopic", "Close the 'General' forum topic"),
        (
            "reopenGeneralForumTopic",
            "Reopen the 'General' forum topic",
        ),
        ("hideGeneralForumTopic", "Hide the 'General' forum topic"),
        (
            "unhideGeneralForumTopic",
            "Unhide the 'General' forum topic",
        ),
        (
            "unpinAllGeneralForumTopicMessages",
            "Clear pinned messages in 'General'",
        ),
        // Stickers
        ("sendSticker", "Send a sticker"),
        ("getStickerSet", "Get a sticker set by name"),
        ("getCustomEmojiStickers", "Get custom emoji stickers by IDs"),
        ("uploadStickerFile", "Upload a sticker file"),
        ("createNewStickerSet", "Create a new sticker set"),
        ("addStickerToSet", "Add a sticker to a set"),
        ("setStickerPositionInSet", "Move a sticker in a set"),
        ("deleteStickerFromSet", "Delete a sticker from a set"),
        ("replaceStickerInSet", "Replace a sticker in a set"),
        ("setStickerSetTitle", "Set the title of a sticker set"),
        (
            "setStickerSetThumbnail",
            "Set the thumbnail of a sticker set",
        ),
        (
            "setCustomEmojiStickerSetThumbnail",
            "Set custom emoji sticker set thumbnail",
        ),
        ("deleteStickerSet", "Delete a sticker set"),
        ("setStickerEmojiList", "Set the emoji list for a sticker"),
        ("setStickerKeywords", "Set search keywords for a sticker"),
        (
            "setStickerMaskPosition",
            "Set the mask position of a sticker",
        ),
        // Inline
        (
            "answerCallbackQuery",
            "Answer a callback query from inline keyboard",
        ),
        ("answerInlineQuery", "Answer an inline query with results"),
        ("answerWebAppQuery", "Answer a Web App query"),
        (
            "savePreparedInlineMessage",
            "Save a prepared inline message",
        ),
        // Payments
        ("sendInvoice", "Send an invoice for payment"),
        ("createInvoiceLink", "Create a link for an invoice"),
        ("answerShippingQuery", "Answer a shipping query"),
        ("answerPreCheckoutQuery", "Answer a pre-checkout query"),
        ("getStarTransactions", "Get the bot's Star transactions"),
        ("refundStarPayment", "Refund a Star payment"),
        (
            "editUserStarSubscription",
            "Edit a user's Star subscription",
        ),
        // Games
        ("sendGame", "Send a game"),
        ("setGameScore", "Set the score of a user in a game"),
        ("getGameHighScores", "Get high score table for a game"),
        // Other
        (
            "setPassportDataErrors",
            "Inform Telegram Passport about data errors",
        ),
        ("getUserProfilePhotos", "Get a list of profile pictures"),
        ("getFile", "Get basic info about a file for downloading"),
        ("getUserChatBoosts", "Get boosts added to a chat by a user"),
        (
            "getBusinessConnection",
            "Get info about a business connection",
        ),
        (
            "getAvailableGifts",
            "Get the list of gifts available for sending",
        ),
        ("sendGift", "Send a gift to a user using Stars"),
        ("setMyProfilePhoto", "Set the bot's profile photo"),
        ("removeMyProfilePhoto", "Remove the bot's profile photo"),
        ("setUserEmojiStatus", "Set a custom emoji status for a user"),
        ("getUserGifts", "Get the gifts received by a user"),
        ("getChatGifts", "Get the gifts received by a chat"),
        ("convertGiftToStars", "Convert a received gift to Stars"),
        ("upgradeGift", "Upgrade a gift to a unique gift"),
        ("transferGift", "Transfer a gift to another user"),
        ("giftPremiumSubscription", "Gift a Premium subscription"),
        ("verifyUser", "Verify a user"),
        ("verifyChat", "Verify a chat"),
        ("removeUserVerification", "Remove user verification"),
        ("removeChatVerification", "Remove chat verification"),
        ("sendChecklist", "Send a checklist message"),
        ("editMessageChecklist", "Edit a checklist message"),
        ("sendMessageDraft", "Send a message draft"),
        ("setChatMemberTag", "Set a custom tag for a chat member"),
        ("getMyStarBalance", "Get the bot's Star balance"),
        ("getUserProfileAudios", "Get profile audios of a user"),
        ("approveSuggestedPost", "Approve a suggested post"),
        ("declineSuggestedPost", "Decline a suggested post"),
        // Business accounts
        ("readBusinessMessage", "Mark a business message as read"),
        ("deleteBusinessMessages", "Delete business account messages"),
        ("setBusinessAccountName", "Set business account name"),
        (
            "setBusinessAccountUsername",
            "Set business account username",
        ),
        ("setBusinessAccountBio", "Set business account bio"),
        (
            "setBusinessAccountProfilePhoto",
            "Set business account photo",
        ),
        (
            "removeBusinessAccountProfilePhoto",
            "Remove business account photo",
        ),
        (
            "setBusinessAccountGiftSettings",
            "Set business gift settings",
        ),
        ("getBusinessAccountStarBalance", "Get business Star balance"),
        ("transferBusinessAccountStars", "Transfer Stars to business"),
        ("getBusinessAccountGifts", "Get business account gifts"),
        // Stories
        ("postStory", "Post a story for a business account"),
        ("editStory", "Edit a story"),
        ("deleteStory", "Delete a story"),
        ("repostStory", "Repost a story"),
    ];

    for (method, desc) in methods {
        println!("{:<42} {}", method, desc);
    }
    println!("\n{} methods total", methods.len());
}

/// Execute a Telegram Bot API method via CLI.
pub async fn call_method(
    token: &str,
    method: &str,
    params_json: Option<&str>,
) -> anyhow::Result<()> {
    let client = BotApiClient::new(token.to_string());

    let params: Value = match params_json {
        Some(json) => {
            serde_json::from_str(json).map_err(|e| anyhow::anyhow!("Invalid JSON: {e}"))?
        }
        None => serde_json::json!({}),
    };

    match client.call_raw(method, &params).await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", serde_json::json!({"error": e}));
            std::process::exit(1);
        }
    }
}
