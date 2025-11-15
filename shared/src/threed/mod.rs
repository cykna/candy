use std::sync::Arc;

use bytemuck::NoUninit;
pub use vello::wgpu::{
    self, PolygonMode, PrimitiveTopology, TextureFormat, VertexBufferLayout, vertex_attr_array,
};
use vello::wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferBinding,
    BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState, FrontFace, IndexFormat,
    MultisampleState, PipelineCompilationOptions, PrimitiveState, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, ShaderModule, ShaderStages, VertexState,
    util::{BufferInitDescriptor, DeviceExt},
};

pub trait GpuCalculation {}

///A Mesh to be rendered by the 3D Scene. This can be SingleObjectMesh, which uses 1 Draw call per mesh, or IndexedMesh, which uses 1 draw call per N meshes via indexing
pub enum Mesh {
    Single(SingleObjectMesh),
    Index(InstancedMesh),
}
impl Mesh {
    ///Creates a new mesh for the given `single` mesh. This will require 1 draw call to be drawn
    pub fn new_single(single: SingleObjectMesh) -> Self {
        Self::Single(single)
    }
    pub fn render(&self, pass: &mut RenderPass) {
        match self {
            Self::Single(single) => single.render(pass),
            Self::Index(instances) => unimplemented!(),
        }
    }
}

pub trait GpuVertex: NoUninit {
    const VERTEX_LAYOUT: VertexBufferLayout<'static>;
    const INSTANCE_LAYOUT: VertexBufferLayout<'static>;
}

pub enum BindGroupType {
    Uniform,
}

///The data of some binding in a group of a shader
pub struct BindGroupData {
    ///The visibility of this data
    pub visibility: ShaderStages,
    ///The type of this data on the shader
    pub bindgroup_ty: BindGroupType,
}

pub struct MaterialData<'a> {
    ///The types of vertices, in order, the vertex shader will receive
    pub vertices: &'a [VertexBufferLayout<'static>],
    ///The module that will be used when rendering the contents
    pub shader: &'a ShaderModule,
    ///The way the shader will be drawn
    pub draw_type: PrimitiveTopology,
    ///Whether this is a right or left handed material. Being right handed will end up by using Ccw and left handed Cw
    pub right_handed: bool,
    ///The way the polygon will be draw
    pub polygon_type: PolygonMode,
    ///The texture format this mesh will work with. Generally use the surface format
    pub texture_format: TextureFormat,
    ///The bindgroup rules for the material. This is specified on the shader
    pub bindgroups_data: Vec<Vec<BindGroupData>>,
}

///A Material that can be used to draw in conjunct with a mesh. A mesh contains vertices uniforms and etc, the material just tells how the mesh will be
///rendered on the screen instead of defining it's contents. It will tell what shader it will use, how that shader will be rendered, etc.
pub struct Material {
    pipeline: RenderPipeline,
    layouts: Vec<BindGroupLayout>,
}

///Defines the resource used by a bindgroup.
pub enum BindGroupResource<'a> {
    Buffer(&'a Buffer),
}

impl<'a> BindGroupResource<'a> {}

///The initial contents used when creating a new mesh
pub struct MeshData<'a, T: GpuVertex> {
    ///The vertices of the mesh
    pub vertices: &'a [T],
    ///The indices of how the vertices should be drawn
    pub indices: &'a [u8],
    ///Whether the index is a u16 or not
    pub indexu16: bool,
    ///The material used when rendering the mesh
    pub material: Arc<Material>,
    ///The bindgroups the mesh will use when rendering
    pub bindgroups: Vec<BindGroup>,
}

impl Material {
    ///Creates a new material using the given `device`. Uses the provided `data` to configure the inner render pipeline. The material will use the entry points
    ///for vertex as "vs_main" and for fragment, "fs_main".
    ///For the provided `bindgroups_data`, internally, this will create their layouts in order they appear. So `bindgroups_data[N][M]` will be on the shader deffition,
    ///of the group N on the binding M.
    pub fn new<'a>(device: &Device, data: MaterialData<'a>) -> Self {
        let layouts = {
            let mut vec = Vec::new();
            for group in data.bindgroups_data.into_iter() {
                let bindgroup_layouts =
                    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                        label: Some("material layout descriptor"),
                        entries: &group
                            .into_iter()
                            .enumerate()
                            .map(|(binding, data)| {
                                let ty = match data.bindgroup_ty {
                                    BindGroupType::Uniform => BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Uniform,
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                };
                                BindGroupLayoutEntry {
                                    binding: binding as u32,
                                    visibility: data.visibility,
                                    ty,
                                    count: None,
                                }
                            })
                            .collect::<Vec<_>>(),
                    });
                vec.push(bindgroup_layouts);
            }
            vec
        };

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("material layout"),
            bind_group_layouts: &layouts.iter().collect::<Vec<_>>(),
            push_constant_ranges: &[],
        });
        Self {
            layouts,
            pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("material pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    module: data.shader,
                    entry_point: Some("vs_main"),
                    compilation_options: PipelineCompilationOptions {
                        constants: &[],
                        zero_initialize_workgroup_memory: false,
                    },
                    buffers: &data.vertices,
                },
                fragment: Some(FragmentState {
                    module: data.shader,
                    entry_point: Some("fs_main"),
                    compilation_options: PipelineCompilationOptions {
                        constants: &[],
                        zero_initialize_workgroup_memory: false,
                    },
                    targets: &[Some(ColorTargetState {
                        format: data.texture_format,
                        write_mask: ColorWrites::ALL,
                        blend: Some(BlendState::REPLACE),
                    })],
                }),
                primitive: PrimitiveState {
                    topology: data.draw_type,
                    strip_index_format: None,
                    front_face: if data.right_handed {
                        FrontFace::Ccw
                    } else {
                        FrontFace::Cw
                    },
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: data.polygon_type,
                    conservative: false,
                },
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                depth_stencil: None,
                cache: None,
                multiview: None,
            }),
        }
    }
    ///Retrieves the inner pipeline that will be used when rendering
    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    ///Creates bindgroups to work properly with this material. The Nth bindgroup correspond to the Nth group on the shader deffinition.
    ///Uses the provided `resources` to create the bindgroups. Note that the Nth index on the Vec will correspond to the Nth bindgroup, and the Mth
    ///in it is the Mth binding. So you can understand it as `group(N) binding(M) = resources[N][M]`.
    ///This one checks mainly for the layouts defined on this material, if the amount of resources passed is smaller than the amount of layouts, it will panic, if it's equal or bigger, it won't be indexed and thus no problem
    pub fn create_bindgroups<'a>(
        &self,
        device: &Device,
        resources: Vec<Vec<BindGroupResource<'a>>>,
    ) -> Vec<BindGroup> {
        let mut out = Vec::with_capacity(self.layouts.len());
        for (index, layout) in self.layouts.iter().enumerate() {
            let bindgroup = device.create_bind_group(&BindGroupDescriptor {
                label: Some("bindgroup creation"),
                layout,
                entries: &resources[index]
                    .iter()
                    .enumerate()
                    .map(|(index, resource)| BindGroupEntry {
                        binding: index as u32,
                        resource: match resource {
                            BindGroupResource::Buffer(buf) => {
                                BindingResource::Buffer(buf.as_entire_buffer_binding())
                            }
                        },
                    })
                    .collect::<Vec<_>>(),
            });
            out.push(bindgroup);
        }
        out
    }
}

///A Mesh to be rendered by the 3D Scene
pub struct SingleObjectMesh {
    ///The buffer containing the vertices of the mesh
    vertices: Buffer,
    ///The indices used when drawing the mesh
    indices: Buffer,
    ///The bindgroups used when drawing. Their group number will correspond to their index on the vector
    bindgroups: Vec<BindGroup>,
    ///The material used to render this mesh
    material: Arc<Material>,
    ///The amount of vertices this mesh contains
    vertices_len: u32,
    ///The amount of indices this mesh contains
    index_len: u32,
    ///The index format that will be used when drawing
    index_format: IndexFormat,
}
impl SingleObjectMesh {
    pub fn new<'a, T: GpuVertex>(device: &Device, data: MeshData<'a, T>) -> Self {
        Self {
            vertices: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data.vertices),
                usage: BufferUsages::VERTEX,
            }),
            indices: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: data.indices,
                usage: BufferUsages::INDEX,
            }),
            index_format: if data.indexu16 {
                IndexFormat::Uint16
            } else {
                IndexFormat::Uint32
            },
            index_len: data.indices.len() as u32 >> if data.indexu16 { 1 } else { 2 },
            vertices_len: data.vertices.len() as u32,
            material: data.material,
            bindgroups: data.bindgroups,
        }
    }
    pub fn render(&self, pass: &mut RenderPass) {
        {
            let mut idx = 0;
            for bindgroup in self.bindgroups.iter() {
                pass.set_bind_group(idx, bindgroup, &[]);
                idx += 1;
            }
        }
        pass.set_pipeline(self.material.pipeline());
        pass.set_index_buffer(self.indices.slice(..), self.index_format);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.draw_indexed(0..self.index_len, 0, 0..1);
    }
}

///A Mesh instance that controls a bunch of ones that will all be rendered via instancing
pub struct InstancedMesh {
    buffer: Buffer,
}
