use bytemuck::{Pod, Zeroable};
use eframe::{
    egui::{self},
    egui_wgpu::{self, RenderState, WgpuConfiguration},
    wgpu::{self, util::DeviceExt, BufferBinding, CommandBuffer, Features},
    Renderer,
};
use space::{deg, point2, rad, vec2, Angle, Point2, TurnDir, Vec2};
use std::{num::NonZeroU64, sync::Arc};

const RENDER_LABEL: Option<&'static str> = Some("Sketch");

#[derive(Debug)]
enum FanKind {
    Start,
    CcwTurn,
    CwTurn,
    End,
}

struct FanVerts {
    verts: Vec<Vertex>,
    indices: Vec<u32>,
    left_start_idx: u32,
    right_start_idx: u32,
    left_end_idx: u32,
    right_end_idx: u32,
}

#[derive(Debug)]
struct Fan {
    kind: FanKind,
    base: Point2,
    radius_center: Point2,
    radius: f64,
    start: Angle,
    end: Angle,
}
impl Fan {
    pub fn new(
        kind: FanKind,
        base: Point2,
        radius_center: Point2,
        radius: f64,
        start: Angle,
        end: Angle,
    ) -> Self {
        Self {
            kind,
            base,
            radius_center,
            radius,
            start: start.normalize(),
            end: end.normalize(),
        }
    }

    fn point_at_angle_from_start(&self, angle: Angle) -> Point2 {
        let angle = self.start + angle;
        self.radius_center + self.radius * vec2(angle.cos(), angle.sin())
    }

    pub fn verts(&self, max_angle: Angle) -> FanVerts {
        println!("\n\nFAN VERTS FOR {:#?}", self);
        let mut vertices = vec![
            self.base.into(),
            self.point_at_angle_from_start(rad(0.0)).into(),
        ];

        let angle = self.start.angle_ccw(self.end);

        //println!("(end - start = angle) -> ({} - {} = {})", end, start, angle);

        if angle.radians() == 0.0 {
            return FanVerts {
                verts: vertices,
                indices: vec![],
                left_start_idx: 0,
                right_start_idx: 1,
                left_end_idx: 0,
                right_end_idx: 1,
            };
        }

        let steps = (angle / max_angle).ceil() as i32;

        let step = angle / steps as f64;

        println!("steps = {}", steps);
        println!("step = {}", step.degrees());

        for i in 1..=steps {
            let angle = step * i as f64;
            println!("step angle = {}", angle);
            vertices.push(self.point_at_angle_from_start(angle).into());
        }

        for v in vertices.iter() {
            println!("{:?}", v.position);
        }

        let verts_len = vertices.len() as u32;

        let indices = (0..verts_len - 2)
            .flat_map(|i| [0, i as u32 + 1, i as u32 + 2])
            .collect::<Vec<u32>>();

        match self.kind {
            FanKind::Start => FanVerts {
                verts: vertices,
                indices,
                left_start_idx: 0,  // unused
                right_start_idx: 0, // unused
                left_end_idx: 0,
                right_end_idx: verts_len - 1,
            },
            FanKind::CcwTurn => FanVerts {
                verts: vertices,
                indices,
                left_start_idx: 0,
                right_start_idx: 1,
                left_end_idx: 0,
                right_end_idx: verts_len - 1,
            },
            FanKind::CwTurn => FanVerts {
                verts: vertices,
                indices,
                left_start_idx: verts_len - 1,
                right_start_idx: 0,
                left_end_idx: 1,
                right_end_idx: 0,
            },
            FanKind::End => FanVerts {
                verts: vertices,
                indices,
                left_start_idx: verts_len - 1,
                right_start_idx: 0,
                left_end_idx: 0,  // unused
                right_end_idx: 0, // unused
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Quad {
    p0: Point2,
    p1: Point2,
    p2: Point2,
    p3: Point2,
}

#[derive(Clone, Debug)]
struct Segment {
    p0: Point2,
    p1: Point2,
}
impl Segment {
    fn new(p0: Point2, p1: Point2) -> Self {
        Self { p0, p1 }
    }

    fn intersect(&self, other: &Segment) -> Option<Point2> {
        let a = self.p0;
        let b = self.p1;
        let c = other.p0;
        let d = other.p1;

        let ax = a.x;
        let ay = a.y;
        let bx = b.x;
        let by = b.y;
        let cx = c.x;
        let cy = c.y;
        let dx = d.x;
        let dy = d.y;

        let den = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);

        let r = ((ay - cy) * (dx - cx) - (ax - cx) * (dy - cy)) / den;

        let s = ((ay - cy) * (bx - ax) - (ax - cx) * (by - ay)) / den;

        if r < 0.0 || r > 1.0 {
            return None;
        }

        if s < 0.0 || s > 1.0 {
            return None;
        }

        let p = a + r * (b - a);

        Some(p)
    }
}

struct Stroke {
    half_width: f64,
    max_angle: Angle,
}
impl Stroke {
    pub fn new(width: f64, max_angle: Angle) -> Self {
        Self {
            half_width: width / 2.0,
            max_angle,
        }
    }
}

#[derive(Debug)]
struct Offsets {
    p0: Point2,
    p1: Point2,
    vec: Vec2,
    orth: Vec2,
    s0: Segment,
    s1: Segment,
}
impl Offsets {
    pub fn new(p0: Point2, p1: Point2, stroke_half_width: f64) -> Self {
        let vec = p1 - p0;
        let orth = vec.orthogonal().normalize() * stroke_half_width;
        let s0 = Segment::new(p0 + orth, p1 + orth);
        let s1 = Segment::new(p0 - orth, p1 - orth);

        Self {
            p0,
            p1,
            vec,
            orth,
            s0,
            s1,
        }
    }
}

struct PolyLine {
    points: Vec<Point2>,
}
impl PolyLine {
    pub fn to_verts(&self, stroke: &Stroke) -> (Vec<Vertex>, Vec<u32>) {
        if self.points.len() < 2 {
            return (vec![], vec![]);
        }

        enum Points {
            FirstTwo(Point2, Point2),
            Middle(Point2, Point2, Point2),
            LastTwo(Point2, Point2),
        }

        let mut fans = vec![];

        for i in 0..self.points.len() {
            let points = if i < 1 {
                Points::FirstTwo(self.points[i], self.points[i + 1])
            } else if i == self.points.len() - 1 {
                Points::LastTwo(self.points[i - 1], self.points[i])
            } else {
                Points::Middle(self.points[i - 1], self.points[i], self.points[i + 1])
            };

            match points {
                Points::FirstTwo(p0, p1) => {
                    // First fan
                    let offsets = Offsets::new(p0, p1, stroke.half_width);
                    let fan = Fan::new(
                        FanKind::Start,
                        offsets.s0.p0,
                        offsets.p0,
                        stroke.half_width,
                        offsets.orth.angle(),
                        (-offsets.orth).angle(),
                    );
                    fans.push(fan);
                }
                Points::Middle(p0, p1, p2) => {
                    // Intermediate fan
                    let offsets0 = Offsets::new(p0, p1, stroke.half_width);
                    let offsets1 = Offsets::new(p1, p2, stroke.half_width);

                    let turn = offsets0.vec.turn_dir(offsets1.vec);

                    println!("TURN = {:?}", turn);

                    let fan = match turn {
                        TurnDir::Cw => Fan::new(
                            FanKind::CwTurn,
                            match offsets0.s1.intersect(&offsets1.s1) {
                                Some(intersection) => intersection,
                                None => ((offsets0.s1.p1.into_vec() + offsets1.s1.p0.into_vec())
                                    / 2.0)
                                    .into_point(),
                            },
                            p1,
                            stroke.half_width,
                            offsets1.orth.angle(),
                            offsets0.orth.angle(),
                        ),
                        TurnDir::Ccw => Fan::new(
                            FanKind::CcwTurn,
                            match offsets0.s0.intersect(&offsets1.s0) {
                                Some(intersection) => intersection,
                                None => ((offsets0.s0.p1.into_vec() + offsets1.s0.p0.into_vec())
                                    / 2.0)
                                    .into_point(),
                            },
                            p1,
                            stroke.half_width,
                            (-offsets0.orth).angle(),
                            (-offsets1.orth).angle(),
                        ),
                        TurnDir::Aligned => Fan::new(
                            FanKind::CcwTurn,
                            offsets0.s0.p1,
                            p1,
                            stroke.half_width,
                            rad(0.0),
                            rad(0.0),
                        ),
                        TurnDir::Opposite => Fan::new(
                            FanKind::CcwTurn,
                            match offsets0.s0.intersect(&offsets1.s0) {
                                Some(intersection) => intersection,
                                None => ((offsets0.s0.p1.into_vec() + offsets0.s1.p0.into_vec())
                                    / 2.0)
                                    .into_point(),
                            },
                            p1,
                            stroke.half_width,
                            offsets0.s1.p1.into_vec().angle(),
                            offsets1.s1.p0.into_vec().angle(),
                        ),
                    };
                    fans.push(fan);
                }
                Points::LastTwo(p0, p1) => {
                    // Last fan
                    let offsets = Offsets::new(p0, p1, stroke.half_width);
                    let fan = Fan::new(
                        FanKind::End,
                        offsets.s1.p1,
                        offsets.p1,
                        stroke.half_width,
                        (-offsets.orth).angle(),
                        offsets.orth.angle(),
                    );
                    fans.push(fan);
                }
            };
        }

        // Turns fans into vertices
        let mut vertices = vec![];
        let mut indices = vec![];

        let mut prev_fan_verts = fans[0].verts(stroke.max_angle);
        vertices.extend(prev_fan_verts.verts.clone());
        indices.extend(prev_fan_verts.indices.clone());

        println!("fans.len() = {}", fans.len());

        for i in 1..fans.len() {
            println!("ITER {}", i);
            let verts_len = vertices.len() as u32;
            let fan_verts = fans[i].verts(stroke.max_angle);

            let prev_fan_index_start = verts_len - prev_fan_verts.verts.len() as u32;

            indices.extend([
                // Triangle 1
                prev_fan_index_start + prev_fan_verts.left_end_idx,
                prev_fan_index_start + prev_fan_verts.right_end_idx,
                verts_len + fan_verts.left_start_idx,
                // Triangle 2
                verts_len + fan_verts.left_start_idx,
                prev_fan_index_start + prev_fan_verts.right_end_idx,
                verts_len + fan_verts.right_start_idx,
            ]);

            vertices.extend(fan_verts.verts.clone());
            indices.extend(fan_verts.indices.iter().map(|i| verts_len + i));

            prev_fan_verts = fan_verts;
        }

        /*
        for i in 0..fans.len() {
            let next_index = vertices.len() as u32;
            let fan = &fans[i];
            let verts = fan.verts(max_angle);

            println!("fan_verts.len() = {}", verts.verts.len());

            let fan_indices = verts
                .indices
                .iter()
                .map(|i| i + next_index)
                .collect::<Vec<u32>>();

            if let Some(last_fan_verts_len) = last_fan_verts_len {
                indices.extend([
                    // First triangle
                    next_index - last_fan_verts_len,
                    next_index - 1,
                    next_index,
                    // Second triangle
                    next_index,
                    next_index - 1,
                    next_index + verts.len() as u32 - 1,
                ]);
            }

            last_fan_verts_len = Some(verts.len() as u32);

            vertices.extend(verts);
            indices.extend(fan_indices);
        }
         */

        println!("vertices.len() = {}", vertices.len());

        println!("indices = {:?}", indices);

        (vertices, indices)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x2
    ];

    fn from_point(point: Point2) -> Self {
        Self {
            position: point.to_f32s(),
        }
    }

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
impl From<Point2> for Vertex {
    fn from(value: Point2) -> Self {
        Self::from_point(value)
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.6],
    },
    Vertex {
        position: [0.6, -0.6],
    },
    Vertex {
        position: [-0.6, -0.6],
    },
];

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let get_device_descriptor = |adapter: &wgpu::Adapter| -> wgpu::DeviceDescriptor<'static> {
        wgpu::DeviceDescriptor {
            features: Features::POLYGON_MODE_LINE,
            ..Default::default()
        }
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 900.0)),
        renderer: Renderer::Wgpu,
        wgpu_options: WgpuConfiguration {
            device_descriptor: Arc::new(get_device_descriptor),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut sketch_pipeline_initialized = false;
    let mut sketches = vec![SketchState::new(0)];

    eframe::run_simple_native("View 2D", options, move |ctx, frame| {
        if !sketch_pipeline_initialized {
            init_sketch_pipeline(frame.wgpu_render_state().unwrap());
        }
        sketch_pipeline_initialized = true;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Some app");
            ui.horizontal(|ui| {
                for sketch in sketches.iter_mut() {
                    ui.allocate_ui([800.0, 800.0].into(), |ui| {
                        ui.sketch(sketch);
                    });
                }
            });

            if ui.button("Add").clicked() {
                sketches.push(SketchState::new(sketches.len()));
            }
        });
    })
}

struct RenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
    uniform_buffers: Vec<wgpu::Buffer>,
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffers: Vec<wgpu::Buffer>,
}

#[derive(Clone)]
struct SketchState {
    buffer_index: usize,
    angle: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}
impl SketchState {
    fn new(buffer_index: usize) -> Self {
        let line = PolyLine {
            points: vec![
                point2(0.0, 0.0),   //
                point2(0.2, 0.0),   //
                point2(0.2, 0.2),   //
                point2(0.0, 0.2),   //
                point2(-0.2, -0.2), //
                point2(0.0, -0.1),  //
                point2(0.2, -0.4),  //
                point2(0.2, -0.6),  //
                point2(-0.2, -0.6), //
                point2(0.6, -0.5),  //

                                    /*
                                    point2(0.4, 0.3), //
                                    point2(0.4, 0.5), //
                                    point2(0.2, 0.5), //
                                    point2(0.5, 0.7), //
                                    point2(0.2, 0.6), //
                                     */
            ],
        };
        let (vertices, indices) = line.to_verts(&Stroke::new(0.075, deg(45.0)));

        Self {
            buffer_index,
            angle: 0.0,
            vertices,
            indices,
        }
    }
}
impl egui_wgpu::CallbackTrait for SketchState {
    fn prepare(
        &self,
        device: &eframe::wgpu::Device,
        queue: &eframe::wgpu::Queue,
        _encoder: &mut eframe::wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<CommandBuffer> {
        let resources: &mut RenderResources = resources.get_mut().unwrap();

        let control_count = self.buffer_index + 1;
        if resources.uniform_buffers.len() < control_count {
            resources.uniform_buffers.push(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: RENDER_LABEL,
                    contents: bytemuck::cast_slice(&[0.0_f32; 4]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                },
            ));

            resources.vertex_buffers.push(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: RENDER_LABEL,
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
                },
            ));

            resources.index_buffers.push(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: RENDER_LABEL,
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::INDEX,
                },
            ));

            resources
                .bind_groups
                .push(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: RENDER_LABEL,
                    layout: &resources.bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(BufferBinding {
                            buffer: &resources.uniform_buffers[self.buffer_index],
                            offset: 0,
                            size: NonZeroU64::new(16),
                        }),
                    }],
                }));
        }

        queue.write_buffer(
            &resources.uniform_buffers[self.buffer_index],
            0,
            &bytemuck::cast_slice(&[self.angle, 0.0, 0.0, 0.0]),
        );

        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = resources.get().unwrap();
        render_pass.set_pipeline(&resources.pipeline);
        render_pass.set_vertex_buffer(0, resources.vertex_buffers[self.buffer_index].slice(..));
        render_pass.set_index_buffer(
            resources.index_buffers[self.buffer_index].slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.set_bind_group(0, &resources.bind_groups[self.buffer_index], &[]);

        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
    }
}

trait SketchUi {
    fn sketch(self, state: &mut SketchState);
}
impl SketchUi for &mut egui::Ui {
    fn sketch(self, state: &mut SketchState) {
        egui::Frame::canvas(self.style()).show(self, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            state.angle += response.drag_delta().x * 0.01;

            ui.painter()
                .add(egui_wgpu::Callback::new_paint_callback(rect, state.clone()));
        });
    }
}

fn init_sketch_pipeline(render_state: &RenderState) {
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
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(render_state.target_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            polygon_mode: wgpu::PolygonMode::Line,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    render_state
        .renderer
        .write()
        .callback_resources
        .insert(RenderResources {
            pipeline,
            bind_group_layout,
            bind_groups: vec![],
            uniform_buffers: vec![],
            vertex_buffers: vec![],
            index_buffers: vec![],
        });
}
