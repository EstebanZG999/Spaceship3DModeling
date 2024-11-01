use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use uniforms::Uniforms;

mod bounding_box;
mod color;
mod framebuffer;
mod fragments;
mod vertex;
mod obj;
mod shader;
mod uniforms;
mod line;

use framebuffer::Framebuffer;
use obj::Obj;

fn main() {
    // Ventana
    let ancho_ventana = 800;
    let alto_ventana = 600;
    let mut ventana = Window::new(
        "3D modeling - Y Wing LookAlike",
        ancho_ventana,
        alto_ventana,
        WindowOptions::default(),
    )
    .unwrap();
    ventana.set_position(500, 500);
    ventana.update();

    // Framebuffer
    let ancho_framebuffer = 800;
    let alto_framebuffer = 600;
    let mut framebuffer = Framebuffer::new(ancho_framebuffer, alto_framebuffer);
    let retardo_frame = Duration::from_millis(16);

    // Objeto
    let objeto = Obj::cargar("assets/jet.obj").expect("Failed to load obj");
    let array_vertices = objeto.obtener_arreglo_vertices();
    let direccion_luz = Vec3::new(1.0, 3.0, -4.0);

    // Variables de vista
    let mut traslacion = Vec3::new(500.0, 500.0, -500.0);
    let mut rotacion = Vec3::new(180.0, 0.0, 0.0);
    let mut escala = 100.0f32;

    // Bucle principal de la ventana
    while ventana.is_open() {
        // Cierre de la ventana
        framebuffer.clear();
        if ventana.is_key_down(Key::Escape) {
            break;
        }

        // Manejo de entrada
        manejar_entrada(&ventana, &mut traslacion, &mut rotacion, &mut escala);
        let matriz_modelo = crear_matriz_modelo(traslacion, escala, rotacion);
        let uniforms = Uniforms {
            model_matrix: matriz_modelo,
            light_dir: direccion_luz,
        };

        // Renderizado
        uniforms::render(&mut framebuffer, &uniforms, &array_vertices);

        ventana
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                ancho_framebuffer,
                alto_framebuffer,
            )
            .unwrap();
        std::thread::sleep(retardo_frame);
    }
}

fn crear_matriz_modelo(traslacion: Vec3, escala: f32, rotacion: Vec3) -> Mat4 {
    let matriz_traslacion = nalgebra_glm::translation(&traslacion);
    let matriz_escala = nalgebra_glm::scaling(&Vec3::new(escala, escala, escala));
    let matriz_rotacion_x = nalgebra_glm::rotation(rotacion.x.to_radians(), &Vec3::x_axis());
    let matriz_rotacion_y = nalgebra_glm::rotation(rotacion.y.to_radians(), &Vec3::y_axis());
    let matriz_rotacion_z = nalgebra_glm::rotation(rotacion.z.to_radians(), &Vec3::z_axis());

    matriz_traslacion * matriz_rotacion_z * matriz_rotacion_y * matriz_rotacion_x * matriz_escala
}

fn manejar_entrada(
    ventana: &Window,
    traslacion: &mut Vec3,
    rotacion: &mut Vec3,
    escala: &mut f32,
) {
    let velocidad_movimiento = 10.0; // Velocidad de movimiento
    let velocidad_rotacion = 0.1;    // Velocidad de rotación
    let velocidad_zoom = 0.5;        // Velocidad de zoom

    // Movimiento de cámara con flechas de dirección
    if ventana.is_key_down(Key::Left) {
        traslacion.x -= velocidad_movimiento; // Mover cámara hacia la izquierda
    }
    if ventana.is_key_down(Key::Right) {
        traslacion.x += velocidad_movimiento; // Mover cámara hacia la derecha
    }
    if ventana.is_key_down(Key::Up) {
        traslacion.y -= velocidad_movimiento; // Mover cámara hacia arriba
    }
    if ventana.is_key_down(Key::Down) {
        traslacion.y += velocidad_movimiento; // Mover cámara hacia abajo
    }

    // Control de rotación
    if ventana.is_key_down(Key::A) {
        rotacion.y += velocidad_rotacion; // Rotar en el eje Y hacia la izquierda
    }
    if ventana.is_key_down(Key::D) {
        rotacion.y -= velocidad_rotacion; // Rotar en el eje Y hacia la derecha
    }
    if ventana.is_key_down(Key::W) {
        rotacion.x += velocidad_rotacion; // Rotar en el eje X hacia arriba
    }
    if ventana.is_key_down(Key::S) {
        rotacion.x -= velocidad_rotacion; // Rotar en el eje X hacia abajo
    }

    // Zoom
    if ventana.is_key_down(Key::Q) {
        *escala += velocidad_zoom; // Acercar (aumentar escala)
    }
    if ventana.is_key_down(Key::E) {
        *escala -= velocidad_zoom; // Alejar (disminuir escala)
    }
}