use super::ogl::{GlVertexArray,GlIndexBuffer,GlMesh};

pub trait Renderable {
    fn get_vao(&self) -> &GlVertexArray;
    fn get_ebo(&self) -> &GlIndexBuffer;
}


impl <'a> Renderable for GlMesh {
    fn get_vao(&self) -> &GlVertexArray {
        &self.vao
    }

    fn get_ebo(&self) -> &GlIndexBuffer {
        &self.ebo
    }
}
