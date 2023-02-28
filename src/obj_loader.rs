use std::fs;
use std::collections::HashMap;

use crate::util;

pub struct ObjMesh {
	pub vertices: Vec<util::Vertex>,
	pub indices: Vec<usize>
	//has_normal, has_texture
}

pub fn load_from_obj(filepath: &str)-> ObjMesh{

	let mut vertices:Vec<util::Vertex> = Vec::new();
	let mut indices:Vec<usize> = Vec::new();
	
	let mut has_texture = false;
	let mut has_normal = false;

	let mut positions:Vec<util::vec4> = Vec::new();
	let mut normals:Vec<util::vec4> = Vec::new();
	let mut tex_coords:Vec<util::vec4> = Vec::new();
	
	//let mut cache: HashMap<util::Vertex, usize> = HashMap::new();
	let mut cache: Vec<util::Vertex> = Vec::new();

	let file = fs::read_to_string(filepath).expect("404: File not found");
	let stream = file.lines();

	for line in stream {
		let mut tokens = line.split_whitespace();
		let command = tokens.next();
		match command{

			Some("#")=>continue,
			
			Some("v")=>{
				let point = util::vec4{
					x: tokens.next().unwrap().parse().unwrap(),
					y: tokens.next().unwrap().parse().unwrap(),
					z: tokens.next().unwrap().parse().unwrap(),
					w: 1.0
				};
				positions.push(point);
			},
			Some("vt")=>{
				has_texture = true;
				//take care of textures here
				/*let normal = util::vec4{
					x: tokens.next().unwrap().parse().unwrap(),
					y: tokens.next().unwrap().parse().unwrap(),
					z: tokens.next().unwrap().parse().unwrap(),
					w: 1.0
				};*/
			},

			Some("vn")=>{
				has_normal = true;
				//take care of normals here
				let normal = util::vec4{
					x: tokens.next().unwrap().parse().unwrap(),
					y: tokens.next().unwrap().parse().unwrap(),
					z: tokens.next().unwrap().parse().unwrap(),
					w: 1.0
				};
				normals.push(normal);
			},

			
			Some("f")=>{
				let t_vertice = [
				tokens.next().unwrap(),
				tokens.next().unwrap(),
				tokens.next().unwrap()
				].map(|index_data| {
					let mut index:usize;
					let mut tex_coord:Option<util::vec4>;
					let mut normal:Option<util::vec4>;
					if has_normal || has_texture{
						let mut iter = index_data.split('/');
						index = iter.next().unwrap().parse::<usize>().unwrap()-1;
						if has_texture{
							let tex_coord_index = iter.next().unwrap().parse::<usize>().unwrap()-1;
							tex_coord = Some(tex_coords[tex_coord_index].copy());
						}
						else {iter.next(); tex_coord = None;}
						if has_normal{
							let normal_index = iter.next().unwrap().parse::<usize>().unwrap()-1;
							normal = Some(normals[normal_index].copy());
						}
						else{normal = None}
						return util::Vertex{pos: positions[index].copy(), normal: normal, tex_coord: tex_coord}
					}
					else{
						index = index_data.parse::<usize>().unwrap()-1;
						return util::Vertex{pos: positions[index].copy(), normal: None, tex_coord:None}
					}
				});
				for i in t_vertice{
					let ind = cache.iter().position(|x| x == &i);
					if let Some(index) = ind{
						indices.push(index);
					}
					else {
						cache.push(i.copy());
						vertices.push(i);
						indices.push(vertices.len()-1);
					}
				}
			},
			
			_=>continue
		}
	}

	//println!("{:?}", vertices.len());
	println!("Model loaded!");
	ObjMesh{vertices, indices}

}
