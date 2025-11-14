use bytemuck::NoUninit;
use candy_macros::Vertex;
use nalgebra::{Vector, Vector2};
use vello::wgpu::{
    self, BufferUsages, IndexFormat, RenderPass, VertexBufferLayout,
    util::{BufferInitDescriptor, DeviceExt},
    vertex_attr_array,
};

pub trait GpuCalculation {}
///A Scene that is used to be rendered. This may contain all the informations about the meshes
pub trait ThreeDScene {
    fn insert_mesh(&mut self, mesh: Mesh);

    fn meshes(&self) -> impl Iterator<Item = &Mesh>;
}

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
}

pub trait GpuVertex: NoUninit {
    const VERTEX_LAYOUT: VertexBufferLayout<'static>;
    const INSTANCE_LAYOUT: VertexBufferLayout<'static>;
}

///The initial contents used when creating a new mesh
pub struct MeshData<'a, T: GpuVertex> {
    ///The vertices of the mesh
    pub vertices: &'a [T],
    ///The indices of how the vertices should be drawn
    pub indices: &'a [u8],
    ///The type of the index
    pub index_type: IndexFormat,
}

impl SingleObjectMesh {
    pub fn new<'a, T: GpuVertex>(device: &wgpu::Device, data: MeshData<'a, T>) -> Self {
        Self {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data.vertices),
                usage: BufferUsages::VERTEX,
            }),
            indices: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: data.indices,
                usage: BufferUsages::INDEX,
            }),
            index_format: data.index_type,
            vertices_len: data.vertices.len() as u32,
        }
    }
    pub fn render(&mut self, pass: &mut RenderPass) {
        pass.set_index_buffer(self.indices.slice(..), self.index_format);
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..self.vertices_len, 0..1);
    }
}

///A Mesh to be rendered by the 3D Scene
pub struct SingleObjectMesh {
    buffer: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_format: IndexFormat,
    vertices_len: u32,
}

///A Mesh instance that controls a bunch of ones that will all be rendered via instancing
pub struct InstancedMesh {
    buffer: wgpu::Buffer,
}
