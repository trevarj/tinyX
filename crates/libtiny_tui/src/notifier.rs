use serde::Deserialize;

use crate::{utils::remove_irc_control_chars, MsgTarget};

#[cfg(feature = "desktop-notifications")]
use notify_rust::Notification;

/// Destktop notification handler
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Notifier {
    /// Notifications are disabled.
    Off,
    /// Generate notifications only for mentions.
    Mentions,
    /// Generate notifications for all messages.
    Messages,
}

impl Default for Notifier {
    fn default() -> Self {
        if cfg!(feature = "desktop-notifications") {
            Notifier::Mentions
        } else {
            Notifier::Off
        }
    }
}

#[cfg(feature = "desktop-notifications")]
fn notify(summary: &str, body: &str) {
    // TODO: Report errors somehow
    let _ = Notification::new().summary(summary).body(body).show();
}

#[cfg(not(feature = "desktop-notifications"))]
fn notify(_summary: &str, _body: &str) {}

impl Notifier {
    pub(crate) fn notify_privmsg(
        &mut self,
        sender: &str,
        msg: &str,
        target: &MsgTarget,
        our_nick: &str,
        mention: bool,
    ) {
        if our_nick == sender {
            return;
        }

        let msg = remove_irc_control_chars(msg);

        match *target {
            MsgTarget::Chan { chan, .. } => {
                if *self == Notifier::Messages || (*self == Notifier::Mentions && mention) {
                    notify(&format!("{} in {}", sender, chan.display()), &msg)
                }
            }
            MsgTarget::User {
                nick: ref nick_sender,
                ..
            } => {
                if *self != Notifier::Off {
                    notify(&format!("{} sent a private message", nick_sender), &msg)
                }
            }
            _ => {}
        }
    }
}
