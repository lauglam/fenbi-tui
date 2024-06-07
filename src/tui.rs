use std::io::{self, stdout, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// 此应用程序中使用的终端类型的类型别名。
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// 设置终端。
pub fn init() -> io::Result<Tui> {
    // 首先，应用程序进入备用屏幕，这是一个辅助屏幕，允许您的应用程序呈现所需的任何内容，而不会干扰 shell 中终端应用程序的正常输出。
    execute!(stdout(), EnterAlternateScreen)?;
    // 接下来，应用程序启用原始模式，这会关闭终端的输入和输出处理。这使您的应用程序可以控制何时将字符打印到屏幕上。
    enable_raw_mode()?;
    // 然后应用程序创建一个后端和 Terminal。
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// 恢复终端。
pub fn restore() -> io::Result<()> {
    // 当应用程序完成时，它需要通过离开备用屏幕并禁用原始模式来恢复终端状态。
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
