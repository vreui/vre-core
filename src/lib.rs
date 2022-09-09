//! vrecore: 威惹 核心引擎 (WebRender)

#![deny(unsafe_code)]

extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate webrender;

// TODO
extern crate winit;

// 模块
#[path = "核心/mod.rs"]
mod 核心;
#[path = "桥/mod.rs"]
mod 桥;
#[path = "窗口/mod.rs"]
mod 窗口;

mod wasm;
mod wr;

// 导出
// TODO

// 全局测试
#[cfg(test)]
mod test;

// TODO 临时代码

use webrender::api::ColorF;

use 窗口::窗;

// 威惹引擎 启动入口
pub fn 启动引擎() {
    // TODO
    println!("vrecore::启动引擎");

    let mut 窗口1 = 窗::new(
        "测试1",
        (1280.0 as f64, 720.0 as f64),
        ColorF::new(0.0, 0.0, 0.3, 1.0),
    );

    println!("进入主循环");
    loop {
        if 窗口1.主循环() {
            break;
        }
    }

    窗口1.清理();
}
