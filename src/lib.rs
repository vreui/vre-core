//! vrecore: 威惹 核心引擎 (WebRender)
#![deny(unsafe_code)]

// 模块
#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "window")]
pub mod window;

// 导出
// TODO

// 全局测试
#[cfg(test)]
mod test;
