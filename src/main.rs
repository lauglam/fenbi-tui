//! 大多数 Ratatui 应用程序中的常见模式是：
//! 1. 初始化终端
//! 2. 循环运行应用程序，直到用户退出应用程序
//! 3. 将终端恢复到原始状态

mod app;
mod events;
mod tui;
mod ui;

/// `main` 函数通过调用 `tui` 模块中的方法来设置终端，然后创建并运行应用程序。
/// 它推迟评估调用 `App::run()` 的结果，直到终端恢复后，以确保在应用程序退出后将任何 `Error`
/// 结果显示给用户。
fn main() -> std::io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new();
    tui::run_app(&mut terminal, &mut app)?;
    tui::restore()?;
    Ok(())
}
