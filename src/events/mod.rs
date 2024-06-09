mod key;
mod mouse;

use crossterm::event::{self, Event, KeyEventKind};

use crate::app::App;

/// 应用程序等待并处理从 crossterm 提供给它的任何事件的地方。
/// 事件触发时更改应用程序状态，后续渲染时根据状态进行相应的渲染。
pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    // event::read 函数会阻塞，直到发生事件为止。
    // 如果您的应用程序需要执行 UI 之外的其他任务，那么它应该通过调用 event::poll
    // 来检查是否存在待处理事件， 并设置适合您的应用程序的合理超时时间。
    // 有关此内容的更多信息将在以后的章节中介绍。
    match event::read()? {
        // 检查该事件是否为按键事件非常重要，因为 crossterm 还会在 Windows
        // 上发出按键释放和重复事件。 检查它是否等于 KeyEventKind::Press
        // 非常重要，否则您的应用程序可能会看到重复的事件（按键按下、按键重复和按键向上）。
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            key::handle_key_event(app, key_event)
        }
        Event::Mouse(mouse_event) => mouse::handle_mouse_event(app, mouse_event),
        _ => Ok(()),
    }
}
