//! 大多数 Ratatui 应用程序中的常见模式是：
//! 1. 初始化终端
//! 2. 循环运行应用程序，直到用户退出应用程序
//! 3. 将终端恢复到原始状态

mod app;
mod events;
mod tui;
mod utils;

/// `main` 函数通过调用 `tui` 模块中的方法来设置终端，然后创建并运行应用程序。
fn main() -> std::io::Result<()> {
    let mut terminal = utils::init()?;
    let mut app = app::App::new();
    utils::run_app(&mut terminal, &mut app)?;
    utils::restore()?;
    Ok(())
}
