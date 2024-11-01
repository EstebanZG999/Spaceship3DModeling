use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::{color::Color, vertex::Vertex};

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn cargar(ruta_archivo: &str) -> Result<Self, tobj::LoadError> {
        let (modelos, _) = tobj::load_obj(ruta_archivo, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let malla = &modelos[0].mesh;

        let vertices: Vec<Vec3> = malla.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normales: Vec<Vec3> = malla.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let coordenadas_textura: Vec<Vec2> = malla.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = malla.indices.clone();

        Ok(Obj {
            vertices,
            normals: normales,
            texcoords: coordenadas_textura,
            indices,
        })
    }

    pub fn obtener_arreglo_vertices(&self) -> Vec<Vertex> {
        let mut arreglo_vertices = Vec::new();
        let color_vertice = Color::from_hex(0x5797ff);
        for i in &self.indices {
            arreglo_vertices.push(Vertex {
                color: color_vertice,
                position: self.vertices[*i as usize],
                normal: self.normals[*i as usize],
                tex_coords: self.texcoords[*i as usize],
                transformed_normal: Vec3::new(0.0, 0.0, 0.0),
                transformed_position: Vec3::new(0.0, 0.0, 0.0),
            });
        }
        arreglo_vertices
    }
}