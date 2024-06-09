use std::io::{stdout, Result, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{app::App, events, ui};

/// 此应用程序中使用的终端类型的类型别名。
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// 设置终端。
pub fn init() -> Result<Tui> {
    // 首先，应用程序进入备用屏幕，这是一个辅助屏幕，允许您的应用程序呈现所需的任何内容，而不会干扰
    // shell 中终端应用程序的正常输出。
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    // 接下来，应用程序启用原始模式，这会关闭终端的输入和输出处理。
    // 这使您的应用程序可以控制何时将字符打印到屏幕上。
    enable_raw_mode()?;
    // 然后应用程序创建一个后端和 Terminal。
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// 恢复终端。
pub fn restore() -> Result<()> {
    // 当应用程序完成时，它需要通过离开备用屏幕并禁用原始模式来恢复终端状态。
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}

/// 大多数应用程序都有一个主循环，一直运行到用户选择退出为止。
/// 循环的每次迭代都会通过调用 `Terminal::draw()` 绘制单个帧，然后更新应用程序的状态。
///
/// 使用新的 run 方法为 App 创建一个 impl 块，该方法将充当应用程序的主循环。
pub fn run_app(terminal: &mut Tui, app: &mut App) -> Result<()> {
    while !app.exit {
        terminal.draw(|frame| ui::render_frame(frame, app))?;
        events::handle_events(app)?;
    }

    Ok(())
}
