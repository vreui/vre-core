//! WebRender 功能封装

use std::rc::Rc;

use webrender::{
    api::{
        units::{DeviceIntRect, DeviceIntSize, LayoutPoint, LayoutRect, LayoutSize},
        ColorF, DocumentId, Epoch, FontInstanceKey, PipelineId, RenderNotifier,
    },
    render_api::{RenderApi, Transaction},
    webrender_api::{
        units::{DevicePixel, LayoutPixel},
        CommonItemProperties, DisplayListBuilder, PrimitiveFlags, RenderReasons, SpaceAndClipInfo,
    },
    DebugCommand, DebugFlags, Renderer, WebRenderOptions,
};

use euclid::{Box2D, Point2D, Scale, Size2D};
use gleam::gl;

pub struct 通知器 {
    // TODO
    //事件代理: EventLoopProxy<()>,
}

impl 通知器 {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderNotifier for 通知器 {
    fn clone(&self) -> Box<dyn RenderNotifier> {
        Box::new(Self {})
    }

    fn wake_up(&self, _需要合成: bool) {
        // TODO
        // #[cfg(not(target_os = "android"))]
        // let _ = self.事件代理.send_event(());
    }

    fn new_frame_ready(&self, _: DocumentId, _scrolled: bool, 需要合成: bool) {
        self.wake_up(需要合成);
    }
}

pub struct 渲染封装 {
    渲染器: Renderer,
    接口: RenderApi,
    管线id: PipelineId,
    文档id: DocumentId,
    世代: Epoch,
    字体例键: Option<FontInstanceKey>,
}

impl 渲染封装 {
    pub fn new(
        窗口大小: (i32, i32),
        背景色: ColorF,
        通知器1: Box<通知器>,
        gl: Rc<dyn gl::Gl>,
    ) -> Self {
        let 选项 = WebRenderOptions {
            clear_color: 背景色,
            ..WebRenderOptions::default()
        };
        let 设备大小 = DeviceIntSize::new(窗口大小.0, 窗口大小.1);

        // 创建 WebRender
        let (渲染器, 发送器) =
            webrender::create_webrender_instance(gl, 通知器1, 选项, None).unwrap();
        let 接口 = 发送器.create_api();
        let 文档id = 接口.add_document(设备大小);

        let 世代 = Epoch(0);
        let 管线id = PipelineId(0, 0);

        //加载字体(文档id, 接口);

        Self {
            渲染器,
            接口,
            管线id,
            文档id,
            世代,
            字体例键: None,
        }
    }

    pub fn 销毁(self) {
        self.渲染器.deinit();
    }

    pub fn 调试p(&self, 名称: &'static str) {
        println!("设置标志 {}", 名称);

        let 命令 = DebugCommand::SetFlags(DebugFlags::PROFILER_DBG);
        self.接口.send_debug_cmd(命令);
    }

    pub fn 渲染(&mut self, 窗口大小: (i32, i32), 像素比例: f32) {
        let 设备大小 = DeviceIntSize::new(窗口大小.0, 窗口大小.1);
        let 布局大小 = 设备大小.to_f32() / Scale::new(像素比例);

        println!("DEBUG: 设备大小 {:?}  布局大小 {:?}", 设备大小, 布局大小);

        let mut 事务 = Transaction::new();

        let mut 构 = self.构造显示列表(布局大小);

        事务.set_display_list(self.世代, None, 布局大小, 构.end());
        事务.set_root_pipeline(self.管线id);
        事务.generate_frame(0, RenderReasons::empty());

        self.接口.send_transaction(self.文档id, 事务);
        self.渲染器.update();
        self.渲染器.render(设备大小, 0).unwrap();
    }

    fn 构造显示列表(&self, 布局大小: Size2D<f32, LayoutPixel>) -> DisplayListBuilder {
        let mut 构 = DisplayListBuilder::new(self.管线id);
        let 空间和剪切 = SpaceAndClipInfo::root_scroll(self.管线id);
        构.begin();

        // TODO
        let 范围 = LayoutRect::from_size(布局大小);
        构.push_simple_stacking_context(
            范围.min,
            空间和剪切.spatial_id,
            PrimitiveFlags::IS_BACKFACE_VISIBLE,
        );

        构.push_rect(
            &CommonItemProperties::new(
                LayoutRect::from_origin_and_size(
                    LayoutPoint::new(100.0, 200.0),
                    LayoutSize::new(100.0, 200.0),
                ),
                空间和剪切,
            ),
            LayoutRect::from_origin_and_size(
                LayoutPoint::new(100.0, 200.0),
                LayoutSize::new(100.0, 200.0),
            ),
            ColorF::new(0.0, 1.0, 0.0, 1.0),
        );

        // TODO add text

        构.pop_stacking_context();
        构
    }
}

fn 加载字体(文档id: DocumentId, 接口: &mut RenderApi) {
    // TODO
    let mut 事务 = Transaction::new();

    let 字体键 = 接口.generate_font_key();
    //事务.add_raw_font(字体键, font_bytes, 0);

    let 字体例键 = 接口.generate_font_instance_key();
    事务.add_font_instance(字体例键, 字体键, 32.0, None, None, Vec::new());

    接口.send_transaction(文档id, 事务);
}
