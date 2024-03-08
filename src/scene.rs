use bevy::ecs::entity;
use bevy::math::vec3;
use bevy::{render::render_resource::PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::mesh::{Indices};
use image::{DynamicImage, GenericImageView};
use std::borrow::Borrow;
use std::{path::Path};
use CS461_SimulationCapstone::FlyCam;
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Copy, Clone)]
struct Chunk{
    position: Vec3,
    remove_flag: bool,
    entity: Entity,
}

//Chunk generation settings
static CHUNK_SIZE: f32 = 50.0;          
static CHUNK_RES: usize = 256;              //todo: have low resolution meshed along with high resolution meshes
static CHUNK_VIEW_DISTANCE: u32 = 16;        //todo: make this mutable

//Used for chunk entity world placement
static mut CREATED_CHUNKS: Vec<Chunk> = Vec::new();     //represents created chunks
static mut NULL_CHUNKS: Vec<Chunk> = Vec::new();        //represents null chunks

//Used for mesh generation (won't need after pre generation, todo: remove)
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

#[derive(Component)]
pub struct ChunkComponent{}

pub fn generate_pre_chunks(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    //lets create a whole bunch of chunks
    unsafe{
        //generate chunk based on image file
        let new_mesh = generate_terrain_chunk("D:\\joshu\\Downloads\\Mountain Range 8k Height Map\\Mountain Range 8k Height Map\\Mountain Range Height Map PNG low.png");
        
        let mesh_handle = meshes.add(new_mesh.clone());

        //generate chunk entities
        for i in 0..(CHUNK_VIEW_DISTANCE * CHUNK_VIEW_DISTANCE){
            let new_transform = Transform::from_translation(vec3(0.0, 0.0, 0.0));

            //create entity
            let chunk_entity = 
            commands.spawn((
                //tag this entity as a chunk with chunk component
                ChunkComponent{},
                PbrBundle{
                    mesh: mesh_handle.clone(),
                    transform: new_transform,
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    ..Default::default()
                }
            )).id();

            //save entity, transform, position, and flag
            NULL_CHUNKS.push(Chunk{
                position: vec3(0.0, 0.0, 0.0),
                remove_flag: false,
                entity: chunk_entity,
            })
        }
        println!("created {num} chunks", num = CHUNK_VIEW_DISTANCE * CHUNK_VIEW_DISTANCE);
    }
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
fn chunk_exists(position: Vec3) -> (bool, usize){
    let mut isit: bool = false;
    let mut index: usize = 0;
    unsafe{
        for chunks in CREATED_CHUNKS.iter() {
            if chunks.position == position{
                isit = true;
                break;
            }
            index = index + 1;
        }
    }
    return (isit, index);
}

pub fn generate_chunks_update(camera_query: Query<(&FlyCam, &Transform), Without<ChunkComponent>>, mut chunk_query: Query<(Entity, &mut Transform), With<ChunkComponent>>){
    unsafe{

        //go through every created chunks and mark them as destroy
        for chunks in CREATED_CHUNKS.iter_mut() {
            chunks.remove_flag = true;
        }

        let item = camera_query.iter().nth(0);
        let d = item.expect("no camera found!");
        let camera = d.0;
        let camera_transform = d.1;

        let position = camera_transform.translation;
        let cp = get_chunk_space_position(position);
    
        let half_chunk = (CHUNK_VIEW_DISTANCE as f32 * 0.5).ceil() as i32;
        let mut chunks_to_create: Vec<Vec3> = Vec::new();

        //loop through chunk box
        for x in 0..(CHUNK_VIEW_DISTANCE) {
            for y in 0..(CHUNK_VIEW_DISTANCE) {
                let cur_x = (x as i32 - half_chunk) as f32 + cp.x;
                let cur_y = (y as i32 - half_chunk) as f32 + cp.z;
                let cur_chunk_pos = Vec3::new(cur_x, 0.0, cur_y);
                let (exists, index) = chunk_exists(cur_chunk_pos);

                //check if this chunk already exists
                if exists {
                    //if it does reset remove flag
                    CREATED_CHUNKS[index].remove_flag = false;
                }
                else{
                    chunks_to_create.push(cur_chunk_pos);
                }
            }
        }

        //loop through all chunks and remove those who have the remove_flag set
        let mut i = CREATED_CHUNKS.len();
        while i > 0 {
            i -= 1;
            // Check condition and remove if necessary
            if CREATED_CHUNKS[i].remove_flag == true {
                println!("Destroying chunk at {}", get_chunk_space_position(CREATED_CHUNKS[i].position));
                let null_chunk = Chunk{
                    entity: CREATED_CHUNKS[i].entity,
                    position: CREATED_CHUNKS[i].position,
                    remove_flag: CREATED_CHUNKS[i].remove_flag,
                };
                CREATED_CHUNKS.remove(i);
                NULL_CHUNKS.push(null_chunk);
            }
        }

        let mut entity_transform_hashmap: HashMap<Entity, Chunk> = HashMap::new();

        //go through the chunks we need to create and use it from null chunks
        for i in 0..chunks_to_create.len() {
            println!("Creating new chunk at {}", chunks_to_create[i]);
            //pop null chunk
            let chunk = NULL_CHUNKS.pop();
            let mut chunk_exp = chunk.expect("null chunk error");

            //update chunk position value
            //update remove flag
            chunk_exp.position = chunks_to_create[i];
            chunk_exp.remove_flag = false;

            entity_transform_hashmap.insert(chunk_exp.entity, chunk_exp);

            //save new chunk
            CREATED_CHUNKS.push(chunk_exp);
        }

        for (entity, mut transform) in chunk_query.iter_mut() {
            if !entity_transform_hashmap.contains_key(entity.borrow()) {
                continue;
            }
            let chunk_data: Option<&Chunk> = entity_transform_hashmap.get(entity.borrow());
            transform.translation = get_world_space_position(chunk_data.expect("this shouldn't happen! no chunk data found!").position);
        }

    }

}