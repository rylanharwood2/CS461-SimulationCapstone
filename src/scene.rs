use bevy::asset::io::file;
use bevy::math::vec3;
use bevy::render::mesh::shape::Cube;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::mesh::Indices;
use image::{DynamicImage, GenericImageView};
use std::process::Command;
use std::{borrow::Borrow, vec};
use std::path::Path;
use CS461_SimulationCapstone::FlyCam;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use bevy::{
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    prelude::*,
};
use std::thread::{self, JoinHandle};
use std::{fs, time};
use dotenv::dotenv;
use std::env;

#[derive(Copy, Clone)]
struct Chunk{
    position: Vec3,
    remove_flag: bool,
    entity: Entity,
}

const INITIAL_HM_PATH: &str = "./assets/images/terrainhm.png";
const HM_HEIGHT: f32 = 30.0;

//Chunk generation settings
static CHUNK_SIZE: f32 = 100.0;          
static CHUNK_RES: usize = 256;              //todo: have low resolution meshed along with high resolution meshes
static CHUNK_VIEW_DISTANCE: u32 = 8;        //todo: make this mutable

//Used for chunk entity world placement
static mut CREATED_CHUNKS: Vec<Chunk> = Vec::new();     //represents created chunks
static mut NULL_CHUNKS: Vec<Chunk> = Vec::new();        //represents null chunks

//used for terrain data fetching
static mut CHUNK_POS_THREAD_HANDLE: Lazy<HashMap<String, JoinHandle<DynamicImage>>> = Lazy::new(|| {
    let map = HashMap::new();
    map
});
static mut CHUNK_DATA_CACHE: Lazy<HashMap<String, DynamicImage>> = Lazy::new(|| {
    let map = HashMap::new();
    map
});
static mut THREAD_COUNT: u32 = 0;

#[derive(Component)]
pub struct SkyBoxComponent {}
#[derive(Component)]
pub struct Sun {}
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 2.0,
        maximum_distance: 300.0,
        ..default()
    }
    .build();

    // light
    commands.spawn((
        Sun{},
        DirectionalLightBundle {
            directional_light: DirectionalLight{
                color: Color::Rgba { red: (1.0), green: (1.0), blue: (1.0), alpha: (1.0) },
                illuminance: 10000.0,
                shadows_enabled: true,
                ..default()
            },
            cascade_shadow_config,
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.610865, 0., 0.)),
            ..default()
        }
    ));

    commands.spawn((
        SkyBoxComponent{},
        PbrBundle {
            mesh: meshes.add(Cube::new(1000.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));

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
fn get_pixel_height_nextzen(height_map: &DynamicImage, x: u32, y: u32) -> f32{
    let (width, height) = height_map.dimensions();
    let x = x.min(width - 1);
    let y = y.min(height - 1);
    let pixel = height_map.get_pixel(x, y);
    let height = (pixel[0] as f32 * 256. + pixel[1] as f32 + pixel[2] as f32 / 256.) - 32768.;
    return height;
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

    let world_normal = Vec3::new(-normal.x, normal.z, -normal.y);

    // Normalize the normal vector
    world_normal.normalize()
}
fn create_terrain_mesh_from_path(path: &str) -> Mesh{
    if path.trim() == "" {
        let (vertices, normals, indices) = generate_mesh_no_height(CHUNK_SIZE, CHUNK_RES);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let indi = Indices::U32(indices);
        mesh.set_indices(Some(indi));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        return mesh;
    }
	let image_path = path; // Replace with the path to your image file
	let img = image::open(&Path::new(image_path)).unwrap();
    return create_terrain_mesh(img);
}
fn create_terrain_mesh(img: DynamicImage) -> Mesh{
	let (vertices, normals, indices) = generate_mesh(img, CHUNK_SIZE, CHUNK_RES, HM_HEIGHT);
	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
	let indi = Indices::U32(indices);
	mesh.set_indices(Some(indi));
	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
	mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    return mesh;
}
fn generate_mesh(texture_height_map: DynamicImage, world_size: f32, n: usize, height_scale: f32) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

	let step = world_size / (n - 1) as f32;

	let (width, height) = texture_height_map.dimensions();
    // println!("Texture size: width: {} height: {}", width, height);
	let ratio_w = (width as f32) / (n as f32);
	let ratio_h = (height as f32) / (n as f32);
    // println!("Texture ratio: width: {} height: {}", ratio_w, ratio_h);

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
fn generate_mesh_no_height(world_size: f32, n: usize) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
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
        let new_mesh = create_terrain_mesh_from_path(INITIAL_HM_PATH);
        
        let mesh_handle = meshes.add(new_mesh.clone());

        //generate chunk entities
        for i in 0..(CHUNK_VIEW_DISTANCE * CHUNK_VIEW_DISTANCE){
            let new_transform = Transform::from_translation(vec3(0.0, 0.0, 0.0));

            let mut mat = StandardMaterial::default();
            mat.perceptual_roughness = 0.5;
            mat.metallic = 0.0;
            mat.base_color = Color::rgb(1.0, 1.0, 1.0);
            mat.emissive = Color::rgb(0.0, 0.0, 0.0);
            mat.fog_enabled = true;

            //create entity
            let chunk_entity = 
            commands.spawn((
                //tag this entity as a chunk with chunk component
                ChunkComponent{},
                PbrBundle{
                    mesh: mesh_handle.clone(),
                    transform: new_transform,
                    material: materials.add(mat),
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

pub fn fetch_terrain_data(chunk_x: i32, chunk_y: i32){
    let z = 6;      //zoom
    let max = f32::powf(2.0, z as f32) - 1.0;
    let tilesize = 256;
    let mut x = (chunk_x as f32 + max * 0.5) as i32;
    let mut y = (chunk_y as f32 + max * 0.5) as i32;
    println!("max {}", max);
    println!("x {}", x);
    println!("y {}", y);

    //xy position that is converted into xy space for map api
    x = i32::clamp(x, 0, max as i32);
    y = i32::clamp(y, 0, max as i32);

    //PUT YOUR OWN API!
    let api_key = env::vars().find(|daw| daw.0 == "Nextzen_API" );
    if api_key.is_none() {
        println!("ERROR! NO API KEY!");
        println!("Read README.md for more details.");
        return;
    }

    let api = api_key.unwrap().1;

    let key = format!("{}_{}", chunk_x, chunk_y);

    unsafe{
        //check if this chunk position already is being worked on by a thread
        if CHUNK_POS_THREAD_HANDLE.contains_key(&key) {
            //do nothing
        }
        else {
            println!("SPAWNING thread terrain data! {}", key);
            THREAD_COUNT = THREAD_COUNT + 1;
            println!("THREADS ALIVE {}", THREAD_COUNT);
            let mky = key.clone();

            //start fetching new data on seperate thread so we dont stall main thread
            let handle = thread::spawn(move || {
                let out_file = format!("./temp/image_{chunk_x}_{chunk_y}.png");             

                //check if we have terrain data loaded already   
                if CHUNK_DATA_CACHE.contains_key(&mky) {
                    println!("found cached data! {}", mky);
                    return CHUNK_DATA_CACHE.remove(&mky).unwrap();
                }

                //then check if a file exists already
                let metadata_result = fs::metadata(out_file.clone());
                match metadata_result {
                    Ok(_) => {
                        println!("found existing terrain data file! {}", mky);
                        let img = image::open(&Path::new(out_file.as_str())).unwrap();
                        return img;
                    },
                    Err(_) => {
                        //no existing file found
                    },
                }

                //if checks fail, we call api to download terrain data

                //API INFO:
                //https://www.nextzen.org/
                //https://developers.nextzen.org/about.html
                //https://developers.nextzen.org/login

                //JOSH API KEY: AEuTnCA5TvKSv-dI8lFQYw
                //https://tile.nextzen.org/tilezen/terrain/v1/{tilesize}/terrarium/{z}/{x}/{y}.png?api_key=your-nextzen-api-key
                //height = (red * 256 + green + blue / 256) - 32768

                //2^z - 1
                //1 -> 1    2
                //2 -> 3    4
                //3 -> 7    8
                //4 -> 15   16
                //5 -> 31   32

                //fetch data from api
                let url = format!("https://tile.nextzen.org/tilezen/terrain/v1/{tilesize}/terrarium/{z}/{x}/{y}.png?api_key={api}");
                println!("Fetching data at: ");
                println!("{url}");
                if !Path::new("./temp/").exists(){
                    fs::create_dir("./temp").unwrap();
                } 
                //this only works on windows
                let output = Command::new("cmd")
                    .args(["/C", format!("wget {url} -O {out_file}").as_str() ])
                    .output()
                    .expect("failed to execute process");
                let ret_str = output.status;
                println!("returned {ret_str}");

                let img = image::open(&Path::new(out_file.as_str())).unwrap();

                let ten_millis = time::Duration::from_millis(1000);
                thread::sleep(ten_millis);

                return img;
            });

            //remember thread handle
            CHUNK_POS_THREAD_HANDLE.insert(key, handle);
        }
    }
}
pub fn handle_terrain_data_threads(){
    unsafe{
        let mut threads_to_remove: Vec<String> = Vec::new();

        for threads in CHUNK_POS_THREAD_HANDLE.iter_mut(){
            //check if thread is finished
            let status = threads.1.is_finished();
            //if the thread is finished, delete thread
            if status {
                threads_to_remove.push(threads.0.to_string());
            }
        }

        for keys in threads_to_remove.iter(){
            //get thread handle
            let handle = CHUNK_POS_THREAD_HANDLE.remove(keys).unwrap();
            let results = handle.join().expect("Image is null, this is a bug!");

            THREAD_COUNT = THREAD_COUNT - 1;
            println!("DELETING thread for {}!", keys);
            println!("THREADS ALIVE {}", THREAD_COUNT);

            //fetch data will remove cached results, so we insert it no matter what
            CHUNK_DATA_CACHE.insert(keys.clone(), results);
        }
    }
}
pub fn generate_chunks_update(camera_query: Query<(&FlyCam, &Transform), Without<ChunkComponent>>, mut chunk_query: Query<(Entity, &mut Transform), With<ChunkComponent>>){
    unsafe{

        //go through every created chunks and mark them as destroy
        for chunks in CREATED_CHUNKS.iter_mut() {
            chunks.remove_flag = true;
        }

        let item = camera_query.iter().nth(0);
        let d = item.expect("no camera found!");
        let camera_transform = *d.1;

        let position = camera_transform.translation;
        //camera position in chunk space
        let cp = get_chunk_space_position(position);
    
        let half_chunk = (CHUNK_VIEW_DISTANCE as f32 * 0.5).ceil() as i32;
        let mut chunks_to_create: Vec<Vec3> = Vec::new();

        //loop through chunk box
        for x in 0..(CHUNK_VIEW_DISTANCE) {
            for y in 0..(CHUNK_VIEW_DISTANCE) {

                //get chunk position in chunk space
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
                // println!("Destroying chunk at {}", get_chunk_space_position(CREATED_CHUNKS[i].position));
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
            // println!("Creating new chunk at {}", chunks_to_create[i]);
            //pop null chunk
            let chunk = NULL_CHUNKS.pop();
            let mut chunk_exp = chunk.expect("null chunk error");

            fetch_terrain_data(chunks_to_create[i].x as i32, chunks_to_create[i].z as i32);

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
pub fn update_sky_box(
    camera_query: Query<&Transform, (With<FlyCam>, Without<SkyBoxComponent>, Without<Sun>)>, 
    mut skybox: Query<&mut Transform, (With<SkyBoxComponent>, Without<Sun>)>,
    mut sun: Query<&mut Transform, With<Sun>>,
    ){

    if camera_query.is_empty() {
        println!("cannot find camera!");
        return;
    }
    if skybox.is_empty() {
        println!("cannot find skybox!");
        return;
    }

    let camera = camera_query.single();
    let mut skybox: Mut<'_, Transform> = skybox.single_mut();
    // let mut sunt = sun.single_mut();
    // let eu = sunt.rotation.to_euler(EulerRot::XYZ);
    // sunt.rotation = Quat::from_euler(EulerRot::XYZ, eu.0 + 0.001, 0.5, eu.2);
    skybox.translation = camera.translation;
}