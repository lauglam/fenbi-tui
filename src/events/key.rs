use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

/// 用于处理按键事件。
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) -> std::io::Result<()> {
    // KeyCode 表示按下了哪个特定键。
    match key_event.code {
        KeyCode::Char('q') => {
            app.exit = true;
        }
        _ => {}
    }
    Ok(())
}
