use crate::config::Colors;
use crate::tui::Termbox;

pub(crate) fn draw(
    tb: &mut Termbox,
    colors: &Colors,
    scroll: i32,
    lines: i32,
    pos_x: i32,
    height: i32,
) {
    // Don't show when at the bottom
    if scroll == 0 {
        return;
    }

    // Calculate where the scrollbar is
    let lines = lines as f32;
    let scroll = scroll as f32;
    let page_size = height as f32;
    let bar_pos = (((lines - scroll) / lines) * page_size) - 1f32;

    for y in 0..height {
        let fg = if y == bar_pos as i32 {
            colors.tab_joinpart.fg
        } else {
            colors.faded.fg
        };
        tb.change_cell(pos_x, y, '|', fg, colors.user_msg.bg);
    }
}
