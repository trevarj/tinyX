use std::panic::Location;

use libtiny_common::{ChanNameRef, MsgTarget};

use crate::msg_area::Layout;
use crate::test_utils::expect_screen;
use crate::tui::TUI;

#[test]
fn test_join_part_overflow() {
    let mut tui = TUI::new_test(21, 4);
    let serv = "irc.server_1.org";
    let chan = ChanNameRef::new("#chan");
    tui.new_server_tab(serv, None);
    tui.set_nick(serv, "osa1");
    tui.new_chan_tab(serv, chan);
    tui.next_tab();
    tui.next_tab();

    let target = MsgTarget::Chan { serv, chan };
    let ts = time::at_utc(time::Timespec::new(0, 0));
    tui.add_nick("12345", Some(ts), &target);
    tui.add_nick("abcde", Some(ts), &target);
    tui.add_nick("hijkl", Some(ts), &target);
    tui.draw();

    #[rustfmt::skip]
    let screen =
        "|00:00 +12345 +abcde  |
         |+hijkl               |
         |osa1:                |
         |< #chan              |";

    expect_screen(screen, &tui.get_front_buffer(), 21, 4, Location::caller());
}

#[test]
fn test_alignment_long_string() {
    let mut tui = TUI::new_test(40, 5);
    tui.set_layout(Layout::Aligned { max_nick_len: 12 });
    let serv = "irc.server_1.org";
    let chan = ChanNameRef::new("#chan");
    tui.new_server_tab(serv, None);
    tui.set_nick(serv, "osa1");
    tui.new_chan_tab(serv, chan);
    tui.next_tab();
    tui.next_tab();

    let target = MsgTarget::Chan { serv, chan };
    let ts = time::at_utc(time::Timespec::new(0, 0));
    tui.add_privmsg(
        "osa1",
        "12345678901234567890123456789",
        ts,
        &target,
        false,
        false,
    );
    tui.draw();

    #[rustfmt::skip]
    let screen =
        "|                                        |
         |00:00         osa1: 1234567890123456789 |
         |                    0123456789          |
         |osa1:                                   |
         |mentions irc.server_1.org #chan         |";

    expect_screen(screen, &tui.get_front_buffer(), 40, 5, Location::caller());
}
