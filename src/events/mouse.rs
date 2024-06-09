use crossterm::event::MouseEvent;

use crate::app::App;

/// 用于处理鼠标事件。
pub fn handle_mouse_event(_app: &mut App, _mouse_event: MouseEvent) -> std::io::Result<()> {
    Ok(())
}
