use std::{f32::consts::PI, num::NonZeroU64};

use eframe::{
    egui,
    egui_wgpu::{self, RenderState},
    wgpu::{self, util::DeviceExt},
    Renderer,
};

const RENDER_LABEL: Option<&'static str> = Some("View2D");

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    let mut name = "Ross".to_owned();

    let mut renderer_initialized = false;

    eframe::run_simple_native("View 2D", options, move |ctx, frame| {
        if !renderer_initialized {
            init_renderer(frame.wgpu_render_state().unwrap());

            renderer_initialized = true;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Some app");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                custom_painting(ui);
            });
        });
    })
}

fn custom_painting(ui: &mut egui::Ui) {
    //println!("custom painting");
    let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        RenderCallback { angle: 0.0 },
    ));

    /*
    ui.painter()
        .add(egui_wgpu::Callback::new_paint_callback(rect, callback));
     */
}

fn init_renderer(render_state: &RenderState) {
    let device = &render_state.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: RENDER_LABEL,
        source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: RENDER_LABEL,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(16),
            },
            count: None,
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: RENDER_LABEL,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: RENDER_LABEL,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(render_state.target_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: RENDER_LABEL,
        contents: bytemuck::cast_slice(&[0.0_f32; 4]),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: RENDER_LABEL,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });

    println!("Initialized renderer");

    render_state
        .renderer
        .write()
        .callback_resources
        .insert(RenderResources {
            pipeline,
            bind_group,
            uniform_buffer,
        });
}

struct RenderCallback {
    angle: f32,
}
impl egui_wgpu::CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        device: &eframe::wgpu::Device,
        queue: &eframe::wgpu::Queue,
        _egui_encoder: &mut eframe::wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<eframe::wgpu::CommandBuffer> {
        println!("preparing callback");
        let resources: &RenderResources = resources.get().unwrap();
        resources.prepare(device, queue, self.angle);
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        println!("painting callback");
        let resources: &RenderResources = resources.get().unwrap();
        resources.paint(render_pass);
    }
}

struct RenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}
impl RenderResources {
    fn prepare(&self, device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        println!("preparing resources");
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[angle, 0.0, 0.0, 0.0]),
        );
    }

    fn paint<'rp>(&'rp self, render_pass: &mut wgpu::RenderPass<'rp>) {
        println!("painting resources");
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
