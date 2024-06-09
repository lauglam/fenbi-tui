mod widgets;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::app::App;

/// 我们在对 App 类型的引用上实现这一点，因为渲染函数不会改变任何状态，
/// 并且我们希望能够在调用绘图后使用该应用程序。
///
/// 渲染函数将创建一个带有标题、底部说明文本和一些边框的块。
/// 使用块内的应用程序状态渲染 `Paragraph` 小部件。
/// 块和段落将占据小部件的整个大小。
///
/// `Frame` 上最重要的方法是 `render_widget()` ，它呈现实现 `Widget` 特征的任何类型，
/// 例如 `Paragraph` 、 `List` 结构实现 `Widget` 特征，以便将与渲染相关的代码组织在一个地方。
/// 这允许我们调用 `Frame::render_widget()` 并将闭包中的应用程序传递给 `Terminal::draw` 。
pub fn render_frame(frame: &mut Frame, _app: &App) {
    let title = Title::from(" 粉笔终端用户界面 ".bold());
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec!["Hello World!".into()])]);

    let paragraph = Paragraph::new(counter_text).centered().block(block);

    frame.render_widget(paragraph, frame.size());
}
