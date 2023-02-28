//Rusterizer: Rasterizer for Rustaceans

mod util;
mod window_handler;
mod renderer;
mod obj_loader;

use std::time;

use util::*;
use window_handler::WindowHandler;

use minifb::Key;

//16:9
const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const ASPECT_RATIO: f32 = WIDTH as f32/HEIGHT as f32;


const FOV_ANGLE:f32 = 1.0472;
const Z_NEAR: f32 = 0.5;		//1./FOV_ANGLE.tan();
const Z_FAR: f32 = 500.0;

fn main() {

	let mut window_handler = WindowHandler::new(WIDTH, HEIGHT);

	let mut delta_time:u128 = 0;

	let mut obj_pos = vec3{x: 0.0, y: 0.0, z: 100.0};

	let mut rad:f32 = 0.0;

    let obj = obj_loader::load_from_obj("res/teapot_1.obj");

    let model_data = obj.vertices;
    let index_buffer = obj.indices;

    while window_handler.window.is_open() && !window_handler.window.is_key_down(Key::Escape) {

  		let now = time::Instant::now();

		let scale = matrix4x4{data: [
			[30.0, 0.0,  0.0,  0.0],
			[0.0,  30.0, 0.0,  0.0],
			[0.0,  0.0,  30.0, 0.0],
			[0.0,  0.0,  0.0, 1.0]
			
			]};

		let translate = matrix4x4{data: [
			[1.0, 0.0, 0.0, obj_pos.x],
			[0.0, 1.0, 0.0, obj_pos.y],
			[0.0, 0.0, 1.0, obj_pos.z],
			[0.0, 0.0, 0.0, 1.0]
			]};

  		let mut rotate = matrix4x4{data: [
  			[1.0, 0.0, 0.0, 0.0],
  			[0.0, 1.0, 0.0, 0.0],
  			[0.0, 0.0, 1.0, 0.0],
  			[0.0, 0.0, 0.0, 1.0]
		]};

		//transform {scale, tate, late}

  		while delta_time > 60{
  			
  			/*rad+=0.1;
  			rotate = matrix4x4{data: [
  				[1.0, 0.0, 0.0, 0.0],
  				[0.0, rad.cos(), -1.*rad.sin(), 0.0],
  				[0.0, rad.sin(), rad.cos(), 0.0],
  				[0.0, 0.0, 0.0, 1.0]
  			]};*/
  			window_handler.clear();
  			delta_time-=60;
  		}


		let mut vertex_buffer: Vec<Vertex> = Vec::new();
		for i in (0..model_data.len()){
			//scale -> translate ->project
			let scaled = scale.matrix_vec_mult(&model_data[i].pos);
			let rotated = rotate.matrix_vec_mult(&scaled);
			let translated = translate.matrix_vec_mult(&rotated);
			let rnormal = rotate.matrix_vec_mult(&model_data[i].normal.as_ref().unwrap());
			vertex_buffer.push(Vertex{pos: translated, normal: Some(rnormal), tex_coord: None});
		}

		let cam_pos: vec3 = vec3{x: 0.0, y: 0.0, z: 0.0};
		

    	let proj = matrix4x4{data: [
    		[Z_NEAR/ASPECT_RATIO, 0.0, 0.0, 0.0],
    		[0.0, Z_NEAR, 0.0, 0.0],
       		[0.0, 0.0, (Z_FAR+Z_NEAR), (Z_NEAR*Z_FAR*-1.)],
    		[0.0, 0.0, 1.0, 0.0]
    		]};

		//for points in polygon
		for i in 0..(index_buffer.len()/3){


			let cam_dir = vec3::normalize(&(&vertex_buffer[index_buffer[(i*3)+1]].pos.to_vec3()-&cam_pos));
			//backface culling
			if (vec3::dot(&vertex_buffer[index_buffer[(i*3)+1]].normal.as_ref().unwrap().to_vec3(), &cam_dir))<0.0
			{

				let mut p1 = proj.matrix_vec_mult(&vertex_buffer[index_buffer[(i*3)+0]].pos).to_vec3();
				let mut p2 = proj.matrix_vec_mult(&vertex_buffer[index_buffer[(i*3)+1]].pos).to_vec3();
				let mut p3 = proj.matrix_vec_mult(&vertex_buffer[index_buffer[(i*3)+2]].pos).to_vec3();

				p1.x = (p1.x+1.0)*(WIDTH/2) as f32;
				p1.y = (p1.y+1.0)*(HEIGHT/2) as f32;
				p1.z = ((p1.z*2.)/(Z_FAR-Z_NEAR))-1.;

				p2.x = (p2.x+1.0)*(WIDTH/2) as f32;
				p2.y = (p2.y+1.0)*(HEIGHT/2) as f32;
				p2.z = ((p2.z*2.)/(Z_FAR-Z_NEAR))-1.;
				
				p3.x = (p3.x+1.0)*(WIDTH/2) as f32;
				p3.y = (p3.y+1.0)*(HEIGHT/2) as f32;
				p3.z = ((p3.z*2.)/(Z_FAR-Z_NEAR))-1.;
				
				//window_handler.set_color(Color(234, 234, 232, 1.0));
				//window_handler.draw_triangle("FILL", &p1, &p2, &p3);
				//window_handler.set_color(Color(0, 0, 0, 1.0));	
				window_handler.draw_triangle("FILL", &p1, &p2, &p3);
			}
		}
		window_handler.render();
		window_handler.clear();
		delta_time += now.elapsed().as_millis();
		//println!("{:?}", delta_time);
	}
}



//eaeae8 rgb(234, 234, 232) //white
//4d4ed8 rgb(77, 78, 216)   //blue
//d74e52 rgb(215, 78, 82)	//red
//f3c452 rgb(235, 125, 52)  //orange
//51e154 rgb(81, 225, 84)   //green
//d5d44c rgb(213, 212, 76) //yellow