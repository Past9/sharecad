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
struct Fan {
    base: Point2,
    radius_center: Point2,
    radius: f64,
    start: Angle,
    end: Angle,
}
impl Fan {
    pub fn new(base: Point2, radius_center: Point2, radius: f64, start: Angle, end: Angle) -> Self {
        Self {
            base,
            radius_center,
            radius,
            start,
            end,
        }
    }

    fn point_at_angle_from_start(&self, angle: Angle) -> Point2 {
        let angle = self.start + angle;
        self.radius_center + self.radius * vec2(angle.cos(), angle.sin())
    }

    pub fn verts(&self, max_angle: Angle) -> Vec<Vertex> {
        let mut vertices = vec![
            self.base.into(),
            self.point_at_angle_from_start(rad(0.0)).into(),
        ];

        let angle = self.end - self.start;

        println!(
            "(end - start = angle) -> ({} - {} = {})",
            self.end, self.start, angle
        );

        if angle.radians() == 0.0 {
            return vertices;
        }

        let steps = (angle / max_angle).ceil() as i32;

        let step = angle / steps as f64;

        println!("step = {}", step.degrees());

        for i in 0..steps {
            let angle = step * i as f64;
            vertices.push(self.point_at_angle_from_start(self.start + angle).into());
        }

        println!("fan verts {:#?}", vertices);

        vertices
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

        let r = ((ax - cy) * (dx - cx) - (ax - cx) * (dy - cy)) / den;

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
}
impl Stroke {
    pub fn new(width: f64) -> Self {
        Self {
            half_width: width / 2.0,
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

            println!("[1.0, 0.0].angle() = {}", vec2(1.0, 0.0).angle().degrees());
            println!("[1.0, 1.0].angle() = {}", vec2(1.0, 1.0).angle().degrees());
            println!("[0.0, 1.0].angle() = {}", vec2(0.0, 1.0).angle().degrees());
            println!(
                "[-1.0, 1.0].angle() = {}",
                vec2(-1.0, 1.0).angle().degrees()
            );
            println!(
                "[-1.0, 0.0].angle() = {}",
                vec2(-1.0, 0.0).angle().degrees()
            );
            println!(
                "[-1.0, -1.0].angle() = {}",
                vec2(-1.0, -1.0).angle().degrees()
            );
            println!(
                "[0.0, -1.0].angle() = {}",
                vec2(0.0, -1.0).angle().degrees()
            );
            println!(
                "[1.0, -1.0].angle() = {}",
                vec2(1.0, -1.0).angle().degrees()
            );

            match points {
                Points::FirstTwo(p0, p1) => {
                    // First fan
                    let offsets = Offsets::new(p0, p1, stroke.half_width);
                    println!("first offsets {:#?}", offsets);
                    let fan = Fan::new(
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

                    let fan = match turn {
                        TurnDir::Cw => Fan::new(
                            match offsets0.s1.intersect(&offsets1.s1) {
                                Some(intersection) => intersection,
                                None => ((offsets0.s1.p1.into_vec() + offsets1.s1.p0.into_vec())
                                    / 2.0)
                                    .into_point(),
                            },
                            p1,
                            stroke.half_width,
                            offsets0.s0.p1.into_vec().angle(),
                            offsets1.s0.p0.into_vec().angle(),
                        ),
                        TurnDir::Ccw => Fan::new(
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
                        TurnDir::Aligned => {
                            Fan::new(offsets0.s0.p1, p1, stroke.half_width, rad(0.0), rad(0.0))
                        }
                        TurnDir::Opposite => Fan::new(
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
                    println!("last offsets {:#?}", offsets);
                    let fan = Fan::new(
                        offsets.s1.p1,
                        offsets.p1,
                        stroke.half_width,
                        offsets.orth.angle(),
                        (-offsets.orth).angle(),
                    );
                    fans.push(fan);
                }
            };
        }

        /*
        for i in 2..self.points.len() {
            let p0 = self.points[i - 2];
            let p1 = self.points[i - 1];
            let p2 = self.points[i];

            let v0 = p1 - p0; // Vector of the first line
            let v1 = p2 - p1; // Vector of the second line

            // Direction of turn from first line to second line
            let turn = v0.turn_dir(v1);

            // Vectors orthonormal to the lines
            let orth0 = v0.orthogonal().normalize() * stroke.half_width;
            let orth1 = v1.orthogonal().normalize() * stroke.half_width;

            // Offset segments of the first line
            let s0 = Segment::new(p0 + orth0, p1 + orth0);
            let s1 = Segment::new(p0 - orth0, p1 - orth0);

            // Offset segments of the second line
            let s2 = Segment::new(p1 + orth1, p2 + orth1);
            let s3 = Segment::new(p1 - orth1, p2 - orth1);

            // First fan
            if i == 2 {
                fans.push(Fan::new(
                    s0.p0,
                    p0,
                    stroke.half_width,
                    orth0.angle(),
                    (-orth0).angle(),
                ));
            }

            // Intermediate fans
            let fan = match turn {
                TurnDir::Cw => Fan::new(
                    match s1.intersect(&s3) {
                        Some(intersection) => intersection,
                        None => ((s1.p1.into_vec() + s3.p0.into_vec()) / 2.0).into_point(),
                    },
                    p1,
                    stroke.half_width,
                    s0.p1.into_vec().angle(),
                    s2.p0.into_vec().angle(),
                ),
                TurnDir::Ccw => Fan::new(
                    match s0.intersect(&s2) {
                        Some(intersection) => intersection,
                        None => ((s0.p1.into_vec() + s1.p0.into_vec()) / 2.0).into_point(),
                    },
                    p1,
                    stroke.half_width,
                    s1.p1.into_vec().angle(),
                    s3.p0.into_vec().angle(),
                ),
                TurnDir::Aligned => Fan::new(s0.p1, p1, stroke.half_width, rad(0.0), rad(0.0)),
                TurnDir::Opposite => Fan::new(
                    match s0.intersect(&s2) {
                        Some(intersection) => intersection,
                        None => ((s0.p1.into_vec() + s1.p0.into_vec()) / 2.0).into_point(),
                    },
                    p1,
                    stroke.half_width,
                    s1.p1.into_vec().angle(),
                    s3.p0.into_vec().angle(),
                ),
            };

            fans.push(fan);

            // Last fan
            if i == self.points.len() - 1 {
                fans.push(Fan::new(
                    s1.p1,
                    p2,
                    stroke.half_width,
                    orth1.angle(),
                    (-orth1).angle(),
                ));
            }
        }
         */

        println!("fans {:#?}", fans);

        // Turns fans into vertices
        let mut vertices = vec![];
        let mut indices = vec![];

        let max_angle = deg(22.5);

        for i in 1..fans.len() {
            let f0 = &fans[i - 1];
            let f1 = &fans[i];

            let f0_verts = f0.verts(max_angle);
            let f1_verts = f1.verts(max_angle);

            let f0_verts_len = f0_verts.len();

            // First fan
            if i == 1 {
                for i in 0..f0_verts_len {
                    let i = i as u32;
                    indices.extend([0, i + 1, i + 2]);
                }
                vertices.extend(f0_verts);
            }

            // Remaining fans
            let all_verts_len = vertices.len();

            // Build a quad between the last fan and the next
            indices.extend([
                (all_verts_len - f0_verts_len) as u32, //0
                (all_verts_len) as u32,                // 2
                (all_verts_len + 1) as u32,            // 3
                (all_verts_len + 1) as u32,            // 3
                (all_verts_len - 1) as u32,            // 1
                (all_verts_len - f0_verts_len) as u32, // 0
            ]);

            // Add verts for next fan
            vertices.extend(f1_verts);
        }

        (vertices, indices)
    }

    pub fn to_vertices(&self) -> (Vec<Vertex>, Vec<u32>) {
        /*
        if self.points.len() < 2 {
            return (vec![], vec![]);
        }

        let mut vertices = vec![];
        let mut indices = vec![];

        for i in 1..self.points.len() {
            let p1 = self.points[i - 1];
            let p2 = self.points[i];

            let vec = p2 - p1;
            let r = vec.orthogonal().normalize() * .half_width;

            vertices.extend([
                Vertex {
                    position: (p1 + r).to_f32s(),
                },
                Vertex {
                    position: (p1 - r).to_f32s(),
                },
                Vertex {
                    position: (p2 + r).to_f32s(),
                },
                Vertex {
                    position: (p2 - r).to_f32s(),
                },
            ]);

            let i_start = (i - 1) as u32 * 4;
            indices.extend([
                i_start + 0,
                i_start + 2,
                i_start + 3,
                i_start + 3,
                i_start + 1,
                i_start + 0,
            ]);
        }

        (vertices, indices)
         */
        todo!()
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
        const WOBBLE: f32 = 0.5;

        let line = PolyLine {
            points: vec![
                point2(0.0, 0.0), //
                point2(0.5, 0.0), //
                                  /*
                                  point2(0.4, 0.3), //
                                  point2(0.4, 0.5), //
                                  point2(0.2, 0.5), //
                                  point2(0.5, 0.7), //
                                  point2(0.2, 0.6), //
                                   */
            ],
            //half_width: 0.06,
        };
        let (vertices, indices) = line.to_verts(&Stroke::new(0.25));

        println!("vertices {:#?}", vertices);

        Self {
            buffer_index,
            angle: 0.0,
            vertices,
            indices,
            //
            /*
            vertices: VERTICES
                .iter()
                .map(|v| Vertex {
                    position: [
                        v.position[0] + rand::random::<f32>() * WOBBLE - WOBBLE / 2.0,
                        v.position[1] + rand::random::<f32>() * WOBBLE - WOBBLE / 2.0,
                    ],
                })
                .collect(),
             */
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

            println!("self.indices {:?}", self.indices);

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
