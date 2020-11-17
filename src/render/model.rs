use super::ogl::{GlMesh,Vertex,Index};
use super::material::Material;
use cgmath::{vec3};
use tobj;
use std::path::Path;
use std::fmt::Debug;
use log::info;

#[derive(Debug)]
pub struct Model {
    pub objects: Vec<Object>
}

#[derive(Debug)]
pub struct Object {
    pub mesh: GlMesh,
    pub material: Material,
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
    /// TODO: Proper error handling (currently panics on most issues)
    /// TODO: Handles textures (currently only verts + normals)
    pub fn with_obj_file<P>(mut self, obj_file_path: P) -> Self 
        where P: AsRef<Path> + Debug {
        let (objs, materials) = tobj::load_obj(&obj_file_path, true).expect(&format!("Failed to load file: {:?}", &obj_file_path));
        let default_material = Material::default();

        let materials: Vec<Material> = materials.iter().map(|material| Material::from(material)).collect();

        let new_objs: Vec<Object> = objs.iter().filter_map(|obj| {
            info!("Loading model {}", obj.name);
            if obj.mesh.normals.len() == 0 {
                // Technically valid, but won't play nice, so ignore for now.
                // TODO: Maybe handle with default/computed normals or something? Relevant for un-lit stuff maybe, eg: flat UI meshes.
                info!("Model {} has no normals! Skipping!", obj.name);
                return None;
            }

            // Based on tobj docs, these should _always_ be true, just documenting here for clarity.
            assert_eq!(obj.mesh.positions.len() % 3, 0);
            assert_eq!(obj.mesh.normals.len() % 3, 0);
            assert_eq!(obj.mesh.positions.len(), obj.mesh.normals.len());
            
            let vertices: Vec<Vertex> = (0..obj.mesh.positions.len()/3).map(|index| {
                // TODO: There _must_ be a cleaner way to do this, but i don't know the itertools/other method to do it.
                let index = index * 3;
                let position = vec3(
                    *obj.mesh.positions.get(index).unwrap(),
                    *obj.mesh.positions.get(index+1).unwrap(),
                    *obj.mesh.positions.get(index+2).unwrap(),
                );
                let normal = vec3(
                    *obj.mesh.normals.get(index).unwrap(),
                    *obj.mesh.normals.get(index+1).unwrap(),
                    *obj.mesh.normals.get(index+2).unwrap(),
                );

                Vertex {
                    position,
                    normal,
                }
            }).collect();

            let indices: Vec<Index> = obj.mesh.indices.iter().map(|index| *index as Index).collect();

            let mesh = GlMesh::from_vertices(vertices, indices);

            let material: Material = obj.mesh
                .material_id
                .and_then(|mat_id| materials.get(mat_id))
                .unwrap_or(&default_material)
                .clone();

            Some(Object {
                mesh,
                material,
            })
        }).collect();

        self.objects.extend(new_objs);

        self
    }

    pub fn build(self) -> Model {
        Model {
            objects: self.objects,
        }
    }
}
