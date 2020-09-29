use super::ogl::{GlMesh,Vertex,Index};
use super::material::Material;
use wavefront_obj::obj::{parse as obj_parse,Primitive,ObjSet};
use wavefront_obj::mtl::{parse as mtl_parse,MtlSet};
use std::collections::HashMap;
use cgmath::{Vector3,vec3};

#[derive(Debug)]
pub struct Model {
    pub objects: Vec<Object>
}

#[derive(Debug)]
pub struct Object {
    mesh: GlMesh,
    material: Material,
}

impl Model {
    pub fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }
}

pub struct ModelBuilder {
    objects: Vec<Object>
}

impl ModelBuilder {
    pub fn new() -> ModelBuilder {
        ModelBuilder {
            objects: Vec::new(),
        }
    }

    pub fn with_object(mut self, mesh: GlMesh, material: Material) -> Self {
        self.objects.push(Object{
            mesh,
            material,
        });

        self
    }

    /// Takes meshes from contents of an obj file
    /// TODO: Proper error handling (currently panics on any issue)
    /// TODO: Handles textures (currently only verts + normals)
    pub fn with_obj_file(mut self, obj_file_content: String, mtl_file_content: String) -> Self {
        let objs: ObjSet = obj_parse(obj_file_content).unwrap();
        let materials: MtlSet = mtl_parse(mtl_file_content).unwrap();

        let material_map = materials.materials
            .iter()
            .map(|mat| {
                let processed_mat = Material::from(mat);

                (mat.name.clone(), processed_mat)
            })
            .fold(HashMap::new(), |mut map, (name, mat)| {
                map.insert(name, mat);

                map
            });
        let default_material = Material::default();

        let all_pos_vertices: Vec<Vector3<f32>> = objs
            .objects
            .iter()
            .flat_map(|obj| obj.vertices.iter())
            .map(|v| vec3(v.x as f32, v.y as f32, v.z as f32))
            .collect();

        let all_norm_vertices: Vec<Vector3<f32>> = objs
            .objects
            .iter()
            .flat_map(|obj| obj.normals.iter())
            .map(|n| vec3(n.x as f32, n.y as f32, n.z as f32,))
            .collect();
        let default_norm: Vector3<f32> = Vector3::unit_x();

        self.objects.extend(objs
            .objects
            .iter()
            .flat_map(|obj| obj.geometry.iter())
            .map(|geometry| {
                let material: Material = geometry
                    .material_name
                    .clone()
                    .map(|mat_name| material_map.get(&mat_name).unwrap())
                    .unwrap_or(&default_material)
                    .clone();

                let (vertices, indices) = geometry
                    .shapes
                    .iter()
                    .fold((Vec::new(), Vec::new()), |(vertices, indices), shape| {
                        let vtns = match shape.primitive {
                            Primitive::Point(v) => [v, v, v],
                            Primitive::Line(v1, v2) => [v1, v2, v2],
                            Primitive::Triangle(v1, v2, v3) => [v1, v2, v3],
                        };

                        let (vertices, indices) = vtns
                            .iter()
                            .fold((Vec::new(), Vec::new()), |(vertices, indices), vtn| {
                                let (global_vertex_index, _maybe_tex_index, maybe_norm_index) = vtn;
                                
                                (vertices, indices)
                            });

                        (vertices, indices)
                    });
                
                let mesh = GlMesh::from_vertices(vertices, indices);

                Object {
                    mesh,
                    material,
                }
            }));

        self
    }

    pub fn build(self) -> Model {
        Model {
            objects: self.objects,
        }
    }
}
