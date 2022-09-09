//! 窗口功能封装

use gleam::gl;
use glutin::{ContextBuilder, GlRequest, NotCurrent, WindowedContext};

use webrender::api::ColorF;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use super::wr::{初始化wr, 渲染数据, 通知器};

// GL 版本
const 请求GL: GlRequest = GlRequest::GlThenGles {
    opengl_version: (3, 2),
    opengles_version: (3, 0),
};

pub struct 窗 {
    名称: &'static str,
    事件循环: Option<EventLoop<()>>,
    语境: Option<WindowedContext<NotCurrent>>,

    wr: Option<渲染数据>,
}

impl 窗 {
    /// 创建窗口
    #[allow(unsafe_code)]
    pub fn new(名称: &'static str, 窗口大小: (f64, f64), 背景色: ColorF) -> Self {
        // TODO 事件循环 可以在多个窗口之间共享
        let 事件循环 = EventLoop::new();

        // 创建窗口
        let 窗口构造器 = WindowBuilder::new()
            .with_title(名称)
            .with_inner_size(LogicalSize::new(窗口大小.0, 窗口大小.1));
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
            事件循环: Some(事件循环),
            语境: Some(语境),
            wr: Some(wr),
        }
    }

    pub fn 主循环(mut self) -> ! {
        let 事件循环 = self.事件循环.take().unwrap();
        let mut 自己 = self;
        let mut 需要重绘 = false;

        事件循环.run(move |事件, _目标, 控制流| {
            //控制流.set_poll();
            控制流.set_wait();

            match 事件 {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("退出");
                        控制流.set_exit();
                    }
                    WindowEvent::Resized(_) => {
                        需要重绘 = true;
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    if 需要重绘 {
                        需要重绘 = false;
                        自己.请求渲染();
                    }
                }
                Event::RedrawRequested(_) => {
                    let mut wr = 自己.wr.take().unwrap();
                    自己.渲染(&mut wr);
                    自己.wr = Some(wr);
                }
                Event::LoopDestroyed => {
                    // 清理
                    自己.wr.take().unwrap().销毁();
                }
                _ => {}
            }
        });
    }

    #[allow(unsafe_code)]
    fn 请求渲染(&mut self) {
        // 当前语境
        let 语境 = unsafe { self.语境.take().unwrap().make_current().unwrap() };

        语境.window().request_redraw();

        // 非当前语境
        self.语境 = Some(unsafe { 语境.make_not_current().unwrap() });
    }

    #[allow(unsafe_code)]
    fn 渲染(&mut self, wr: &mut 渲染数据) {
        // 当前语境
        let 语境 = unsafe { self.语境.take().unwrap().make_current().unwrap() };

        let 窗口大小 = {
            let 大小 = 语境.window().inner_size();
            (大小.width as i32, 大小.height as i32)
        };
        let 像素比例 = 语境.window().scale_factor() as f32;

        println!("DEBUG: 窗口大小 {:?}  像素比例 {:?}", 窗口大小, 像素比例);

        wr.渲染(窗口大小, 像素比例);

        语境.swap_buffers().ok();
        // 非当前语境
        self.语境 = Some(unsafe { 语境.make_not_current().unwrap() });
    }
}
