//! 窗口功能封装

use gleam::gl;
use glutin::{ContextBuilder, GlRequest, NotCurrent, WindowedContext};

use webrender::api::ColorF;

// TODO
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use super::wr::{初始化wr, 渲染数据, 通知器};

// GL 版本
const 请求GL: GlRequest = GlRequest::GlThenGles {
    opengl_version: (3, 2),
    opengles_version: (3, 0),
};

pub struct 窗 {
    名称: &'static str,
    事件循环: EventLoop<()>,
    语境: Option<WindowedContext<NotCurrent>>,

    wr: 渲染数据,
}

impl 窗 {
    /// 创建窗口
    #[allow(unsafe_code)]
    pub fn new(名称: &'static str, 窗口大小: (f64, f64), 背景色: ColorF) -> Self {
        // 创建窗口
        let 窗口构造器 = WindowBuilder::new()
            .with_title(名称)
            .with_inner_size(LogicalSize::new(窗口大小.0, 窗口大小.1));

        let 事件循环 = EventLoop::new();
        // 请求 OpenGL / OpenGL ES
        let 语境 = ContextBuilder::new()
            .with_gl(请求GL)
            .build_windowed(窗口构造器, &事件循环)
            .unwrap();

        // 当前语境
        let 语境 = unsafe { 语境.make_current().unwrap() };

        // 初始化 GL
        let gl = match 语境.get_api() {
            glutin::Api::OpenGl => unsafe {
                gl::GlFns::load_with(|符号| 语境.get_proc_address(符号) as *const _)
            },
            glutin::Api::OpenGlEs => unsafe {
                gl::GlesFns::load_with(|符号| 语境.get_proc_address(符号) as *const _)
            },
            glutin::Api::WebGl => unimplemented!(),
        };

        // 创建 WebRender
        let 窗口大小 = {
            let 大小 = 语境.window().inner_size();
            (大小.width as i32, 大小.height as i32)
        };
        let 通知器1 = Box::new(通知器::new(事件循环.create_proxy()));
        let wr = 初始化wr(窗口大小, 背景色, 通知器1, gl.clone());

        // 非当前语境
        let 语境 = unsafe { 语境.make_not_current().unwrap() };

        Self {
            名称,
            事件循环,
            语境: Some(语境),
            wr,
        }
    }

    // 销毁
    pub fn 清理(self) {
        self.wr.销毁();
    }

    #[allow(unsafe_code)]
    pub fn 主循环(&mut self) -> bool {
        if self.处理事件循环() {
            // 退出
            return true;
        }

        // 当前语境
        let 语境 = unsafe { self.语境.take().unwrap().make_current().unwrap() };

        let 窗口大小 = {
            let 大小 = 语境.window().inner_size();
            (大小.width as i32, 大小.height as i32)
        };
        let 像素比例 = 语境.window().scale_factor() as f32;

        self.wr.渲染(窗口大小, 像素比例);

        语境.swap_buffers().ok();
        // 非当前语境
        self.语境 = Some(unsafe { 语境.make_not_current().unwrap() });

        false
    }

    fn 处理事件循环(&mut self) -> bool {
        let 名称 = &self.名称;
        let mut 退出标志 = false;
        let wr = &mut self.wr;

        self.事件循环.run_return(|全局事件, _elwt, 控制流| {
            *控制流 = ControlFlow::Exit;

            match 全局事件 {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        退出标志 = true;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::P),
                                ..
                            },
                        ..
                    } => {
                        wr.调试p(名称);
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        退出标志
    }
}
