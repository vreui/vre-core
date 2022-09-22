//! 窗口功能封装

use gleam::gl;
use webrender::api::ColorF;

use glw::{窗口, 窗口创建参数};

use super::wr::{渲染封装, 通知器};

pub struct 窗 {
    标题: &'static str,
    // TODO
    //事件循环: Option<EventLoop<()>>,
    内部: 窗口,
    渲染: 渲染封装,
}

impl 窗 {
    /// 创建窗口
    pub fn new(
        标题: &'static str, 窗口大小: (f32, f32), 背景色: (f32, f32, f32, f32)
    ) -> Self {
        let 内部 = 窗口::new(窗口创建参数 {
            标题,
            大小: (窗口大小.0 as i32, 窗口大小.1 as i32),
            背景色,
            gl: true,
        });

        let gl = 内部.取gl().unwrap();
        // DEBUG
        println!("GL version {}", gl.get_string(gl::VERSION));
        println!("GL vendor {}", gl.get_string(gl::VENDOR));
        println!("GL renderer {}", gl.get_string(gl::RENDERER));

        // 创建 WebRender
        // TODO
        let 通知器1 = Box::new(通知器::new());

        let 窗口大小 = 内部.取大小();
        let 背景色 = ColorF::new(背景色.0, 背景色.1, 背景色.2, 背景色.3);
        let 渲染 = 渲染封装::new(窗口大小, 背景色, 通知器1, gl.clone());

        Self {
            标题, 内部, 渲染
        }
    }

    pub fn 主循环(&mut self) {
        self.内部.主循环();
    }

    pub fn 清理(self) {
        self.渲染.销毁();
        self.内部.清理();
    }

    // TODO
    // pub fn 主循环(mut self) -> ! {
    //     let 事件循环 = self.事件循环.take().unwrap();
    //     let mut 自己 = self;
    //     // 第一次绘制
    //     let mut 需要重绘 = true;

    //     事件循环.run(move |事件, _目标, 控制流| {
    //         //控制流.set_poll();
    //         控制流.set_wait();

    //         match 事件 {
    //             Event::WindowEvent { event, .. } => match event {
    //                 WindowEvent::CloseRequested => {
    //                     println!("退出");
    //                     控制流.set_exit();
    //                 }
    //                 WindowEvent::Resized(_) => {
    //                     需要重绘 = true;
    //                 }
    //                 _ => {}
    //             },
    //             Event::MainEventsCleared => {
    //                 if 需要重绘 {
    //                     需要重绘 = false;
    //                     自己.请求渲染();
    //                 }
    //             }
    //             Event::RedrawRequested(_) => {
    //                 let mut wr = 自己.wr.take().unwrap();
    //                 自己.渲染(&mut wr);
    //                 自己.wr = Some(wr);
    //             }
    //             Event::LoopDestroyed => {
    //                 // 清理
    //                 自己.wr.take().unwrap().销毁();
    //             }
    //             _ => {}
    //         }
    //     });
    // }

    // fn 请求渲染(&mut self) {
    //     语境.window().request_redraw();
    // }

    // fn 渲染(&mut self, wr: &mut 渲染数据) {
    //     let 窗口大小 = {
    //         let 大小 = 语境.window().inner_size();
    //         (大小.width as i32, 大小.height as i32)
    //     };
    //     let 像素比例 = 语境.window().scale_factor() as f32;

    //     println!("DEBUG: 窗口大小 {:?}  像素比例 {:?}", 窗口大小, 像素比例);

    //     wr.渲染(窗口大小, 像素比例);

    //     语境.swap_buffers().ok();
    // }
}
