//! window: 窗口进程

extern crate euclid;
extern crate gleam;
extern crate glw;
extern crate webrender;

// 模块
#[path = "窗口/mod.rs"]
mod 窗口;

mod wr;

// TODO 临时代码

use 窗口::窗;

// 威惹引擎 启动入口
pub fn 启动引擎() {
    // TODO
    println!("vrecore::启动引擎");

    let mut 窗1 = 窗::new("测试1", (1280.0, 720.0), (0.0, 0.0, 0.3, 1.0));

    println!("进入主循环");
    窗1.主循环();

    窗1.清理();
}

// TODO
