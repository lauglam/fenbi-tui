//! 实现 `App` 结构体，保存应用程序状态。

pub struct App {
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App { exit: false }
    }
}
