use bevy::{ecs::system::EntityCommand, render::render_resource::PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use image::{DynamicImage, FlatSamples, GenericImageView, Pixel};
use std::{path::Path, vec};
use CS461_SimulationCapstone::FlyCam;
use std::collections::HashMap;
use once_cell::sync::Lazy;
struct Chunk{
    position: Vec3,
    remove_flag: bool,
    entity: Entity,
}

static mut CHUNK_SIZE: f32 = 25.0;
static mut CHUNK_RES: usize = 128;
static mut CHUNK_VIEW_DISTANCE: u32 = 8;
static mut OPEN_CHUNKS: Vec<Chunk> = Vec::new();
static mut CHUNK_LOCATION_CACHE: Lazy<HashMap<String, Mesh>> = Lazy::new(|| {
    let map = HashMap::new();
    map
});
static mut FLAT_PLANE_CREATED: bool = false;
static mut FLAT_PLANE_MESH: Option<Mesh> = None;

pub fn setup(
    mut commands: Commands,
) {
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight{
            color: Color::Rgba { red: (1.0), green: (1.0), blue: (1.0), alpha: (1.0) },
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.2, -1.2, 0.)),
        ..Default::default()
    });
    
    info!("Move camera around by using WASD for lateral movement");
    info!("Use Left Shift and Spacebar for vertical movement");
    info!("Use the mouse to look around");
    info!("Press Esc to hide or show the mouse cursor");
}

fn get_pixel_height(height_map: &DynamicImage, x: u32, y: u32) -> f32 {
    let (width, height) = height_map.dimensions();
    let x = x.min(width - 1);
    let y = y.min(height - 1);
    let pixel = height_map.get_pixel(x, y);
    pixel[0] as f32 / 255.0 // Normalize to range 0-1
}
fn compute_world_space_normal(height_map: &DynamicImage, x: u32, y: u32) -> Vec3 {
    // Sample neighboring heights
    let left_height = get_pixel_height(height_map, x.saturating_sub(1), y);
    let right_height = get_pixel_height(height_map, x + 1, y);
    let up_height = get_pixel_height(height_map, x, y.saturating_sub(1));
    let down_height = get_pixel_height(height_map, x, y + 1);

    let dx = right_height - left_height;
    let dy = up_height - down_height;

    let scale_x = 1.0 / height_map.dimensions().0 as f32;
    let scale_y = 1.0 / height_map.dimensions().1 as f32;

    let va = Vec3::new(scale_x, 0.0, dx).normalize();
    let vb = Vec3::new(0.0, scale_y, dy).normalize();

    let normal = va.cross(vb);

    let world_normal = Vec3::new(normal.x, normal.z, normal.y);

    // Normalize the normal vector
    world_normal.normalize()
}
fn generate_terrain_chunk(path: &str) -> Mesh{

    let n;
    let chunk_size;

    unsafe{
        n = CHUNK_RES;
        chunk_size = CHUNK_SIZE;
    }

    if path.trim() == "" {
        println!("No image data provided to terrain, creating flat terrain...");

        unsafe{
            if FLAT_PLANE_CREATED == false{
                println!("Caching mesh with path {}", path);
                let (vertices, normals, indices) = generate_quad_empty(chunk_size, n);
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                let indi = Indices::U32(indices);
                mesh.set_indices(Some(indi));
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                FLAT_PLANE_CREATED = true;
                FLAT_PLANE_MESH = Some(mesh.clone());
                return mesh;
            }
            else{
                println!("Found mesh with path {}", path);
                match FLAT_PLANE_MESH.clone() {
                    Some(value) => return value,
                    None => println!("No value"),
                }
            }
        }
    }

    unsafe{
        if let Some(value) = CHUNK_LOCATION_CACHE.get(path){
            println!("Found mesh with path {}", path);
            return value.clone();
        }
    }

	let image_path = path; // Replace with the path to your image file
	let img = image::open(&Path::new(image_path)).unwrap();

	let (vertices, normals, indices) = generate_quad(img, chunk_size, n, chunk_size);
	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
	let indi = Indices::U32(indices);
	mesh.set_indices(Some(indi));
	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
	mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    unsafe{
        println!("Caching mesh with path {}", path);
        CHUNK_LOCATION_CACHE.insert(path.to_string(), mesh.clone());
    }
    return mesh;
}
fn generate_quad(texture_height_map: DynamicImage, world_size: f32, n: usize, height_scale: f32) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

	let step = world_size / (n - 1) as f32;

	let (width, height) = texture_height_map.dimensions();
    println!("Texture size: width: {} height: {}", width, height);
	let ratio_w = (width as f32) / (n as f32);
	let ratio_h = (height as f32) / (n as f32);
    println!("Texture ratio: width: {} height: {}", ratio_w, ratio_h);

    // Generate vertices for a nxn quad
    for y in 0..n {
        for x in 0..n {

            let pixel_x = (x as f32) * ratio_w;
            let pixel_y = (y as f32) * ratio_h;
            //apply billinear filtering
            //for now use nearest filtering
            let pixel_x_int = pixel_x.floor() as u32;
            let pixel_y_int = pixel_y.floor() as u32;

            //use red channel for vertex height map
            let height = get_pixel_height(&texture_height_map, pixel_x_int, pixel_y_int);
            let normal = compute_world_space_normal(&texture_height_map, pixel_x_int, pixel_y_int);

            // Calculate position for each vertex
            let position = Vec3::new(
				x as f32 * step  - world_size / 2.0, 
				height * height_scale,
				y as f32 * step - world_size / 2.0);
            
            vertices.push(position);
            normals.push(normal);

            // Create indices for the quad
            if x < n - 1 && y < n - 1 {
                let index = (y * n + x) as u32;
                let next_row_index = ((y + 1) * n + x) as u32;
                let next_column_index = (y * n + x + 1) as u32;
                let next_diagonal_index = ((y + 1) * n + x + 1) as u32;

                // Triangle 1
                indices.push(index);
                indices.push(next_row_index);
                indices.push(next_diagonal_index);

                // Triangle 2
                indices.push(index);
                indices.push(next_diagonal_index);
                indices.push(next_column_index);
            }
        }
    }

    (vertices, normals, indices)
}
fn generate_quad_empty(world_size: f32, n: usize) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

	let step = world_size / (n - 1) as f32;

    // Generate vertices for a nxn quad
    for y in 0..n {
        for x in 0..n {
            // Calculate position for each vertex
            let position = Vec3::new(
				x as f32 * step  - world_size / 2.0, 
				0.0,
				y as f32 * step - world_size / 2.0);
            let normal = Vec3::new(0.0, 1.0, 0.0);
            vertices.push(position);
            normals.push(normal);
            // Create indices for the quad
            if x < n - 1 && y < n - 1 {
                let index = (y * n + x) as u32;
                let next_row_index = ((y + 1) * n + x) as u32;
                let next_column_index = (y * n + x + 1) as u32;
                let next_diagonal_index = ((y + 1) * n + x + 1) as u32;

                // Triangle 1
                indices.push(index);
                indices.push(next_row_index);
                indices.push(next_diagonal_index);

                // Triangle 2
                indices.push(index);
                indices.push(next_diagonal_index);
                indices.push(next_column_index);
            }
        }
    }

    (vertices, normals, indices)
}

pub fn setup_terrain(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>){
	// let mesh = generate_terrain_chunk("D:\\joshu\\Downloads\\Mountain Range 8k Height Map\\Mountain Range 8k Height Map\\Mountain Range Height Map PNG low.png");
	// let entity = commands.spawn(PbrBundle {
    //     mesh: meshes.add(mesh),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // }).id();
}

fn get_chunk_space_position(position: Vec3) -> Vec3{
    let x;
    let y;
    let z;
    unsafe{
        x = (position.x / (CHUNK_SIZE)).round();
        y = (position.y / (CHUNK_SIZE)).round();
        z = (position.z / (CHUNK_SIZE)).round();
    }
    Vec3::new(x, y, z)
}
fn get_world_space_position(position: Vec3) -> Vec3{
    let x;
    let y;
    let z;
    unsafe{
        x = (position.x * CHUNK_SIZE).round();
        y = (position.y * CHUNK_SIZE).round();
        z = (position.z * CHUNK_SIZE).round();
    }
    Vec3::new(x, y, z)
}
fn is_in_open_chunks(position: Vec3) -> (bool, usize){
    let mut isit: bool = false;
    let mut index: usize = 0;
    unsafe{
        for chunks in OPEN_CHUNKS.iter() {
            if chunks.position == position{
                isit = true;
                break;
            }
            index = index + 1;
        }
    }
    return (isit, index);
}

pub fn generate_chunks_update(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&FlyCam, &mut Transform)>)
    {
    unsafe{

        for chunks in OPEN_CHUNKS.iter_mut() {
            chunks.remove_flag = true;
        }

        for (_camera, transform) in query.iter() {
            let position = transform.translation;
            let cp = get_chunk_space_position(position);
    
            let half_chunk = (CHUNK_VIEW_DISTANCE as f32 * 0.5).ceil() as i32;

            //loop through chunk box
            for x in 0..(CHUNK_VIEW_DISTANCE + 1) {
                for y in 0..(CHUNK_VIEW_DISTANCE + 1) {
                    let cur_x = (x as i32 - half_chunk) as f32 + cp.x;
                    let cur_y = (y as i32 - half_chunk) as f32 + cp.z;
                    let cur_chunk_pos = Vec3::new(cur_x, 0.0, cur_y);
                    let (exists, index) = is_in_open_chunks(cur_chunk_pos);

                    //check if this chunk already exists
                    if exists {
                        //if it does reset remove flag
                        OPEN_CHUNKS[index].remove_flag = false;
                    }
                    else{
                        //else create new chunk and cache it
                        println!("Creating new chunk at {}", cur_chunk_pos);
                        let mesh = generate_terrain_chunk("D:\\joshu\\Downloads\\Mountain Range 8k Height Map\\Mountain Range 8k Height Map\\Mountain Range Height Map PNG low.png");
                        //let mesh = generate_terrain_chunk("");
                        let new_entity = commands.spawn(PbrBundle {
                            mesh: meshes.add(mesh),
                            transform: Transform::from_translation(get_world_space_position(cur_chunk_pos)),
                            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                            ..Default::default()
                        }).id();

                        let new_chunk = Chunk{
                            position: cur_chunk_pos, 
                            remove_flag: false,
                            entity: new_entity
                        };

                        OPEN_CHUNKS.push(new_chunk);
                    }
                }
            }
        }

        //loop through all chunks and remove those who have the remove_flag set
        let mut i = OPEN_CHUNKS.len();
        while i > 0 {
            i -= 1;
            // Check condition and remove if necessary
            if OPEN_CHUNKS[i].remove_flag == true {
                println!("Destroying chunk at {}", get_chunk_space_position(OPEN_CHUNKS[i].position));
                commands.entity(OPEN_CHUNKS[i].entity).despawn();
                OPEN_CHUNKS.remove(i);
            }
        }
    }

}