//! 实现 `App` 结构体，用于处理 UI 逻辑，保存应用程序状态。

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use color_eyre::{eyre::WrapErr, Result};

use crate::tui;

/// 用于表示应用程序的状态。
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

/// 首先，添加一个新的 `impl Widget for &App` 块。
/// 我们在对 App 类型的引用上实现这一点，因为渲染函数不会改变任何状态，并且我们希望能够在调用绘图后使用该应用程序。
///
/// 渲染函数将创建一个带有标题、底部说明文本和一些边框的块。
/// 使用块内的应用程序状态渲染 `Paragraph` 小部件。
/// 块和段落将占据小部件的整个大小。
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" 粉笔终端界面 ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec!["Hello World!".into()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

/// 存放所有事件和事件相关的方法。
/// 事件触发时更改应用程序状态，后续渲染时根据状态进行相应的渲染。
impl App {
    /// 用于处理按键事件。
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        // KeyCode 表示按下了哪个特定键。
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    /// 用于处理鼠标事件。
    fn handle_mouse_event(&mut self, _mouse_event: MouseEvent) -> Result<()> {
        Ok(())
    }

    /// 设置 `exit` 状态，当下一次循环时，将会退出程序。
    fn exit(&mut self) {
        self.exit = true;
    }
}

/// 大多数应用程序都有一个主循环，一直运行到用户选择退出为止。
/// 循环的每次迭代都会通过调用 `Terminal::draw()` 绘制单个帧，然后更新应用程序的状态。
///
/// 使用新的 run 方法为 App 创建一个 impl 块，该方法将充当应用程序的主循环。
impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }

        Ok(())
    }

    /// 为了呈现 UI，应用程序使用接受 `Frame` 的闭包调用 `Terminal::draw()` 。
    /// `Frame` 上最重要的方法是 `render_widget()` ，它呈现实现 `Widget` 特征的任何类型，
    /// 例如 `Paragraph` 、 `List` 结构实现 `Widget` 特征，以便将与渲染相关的代码组织在一个地方。
    /// 这允许我们调用 `Frame::render_widget()` 并将闭包中的应用程序传递给 `Terminal::draw` 。
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// 应用程序等待并处理从 crossterm 提供给它的任何事件的地方。
    fn handle_events(&mut self) -> Result<()> {
        // event::read 函数会阻塞，直到发生事件为止。
        // 如果您的应用程序需要执行 UI 之外的其他任务，那么它应该通过调用 event::poll 来检查是否存在待处理事件，
        // 并设置适合您的应用程序的合理超时时间。有关此内容的更多信息将在以后的章节中介绍。
        match event::read()? {
            // 检查该事件是否为按键事件非常重要，因为 crossterm 还会在 Windows 上发出按键释放和重复事件。
            // 检查它是否等于 KeyEventKind::Press 非常重要，否则您的应用程序可能会看到重复的事件（按键按下、按键重复和按键向上）。
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            Event::Mouse(mouse_event) => self
                .handle_mouse_event(mouse_event)
                .wrap_err_with(|| format!("handling mouse event failed:\n{mouse_event:#?}")),
            _ => Ok(()),
        }
    }
}
