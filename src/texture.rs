use std::fs;
use crate::util;

//Sprite.rs

//This isn't an actual texture since its on CPU
pub struct Texture {
	pub width: usize,
	pub height: usize,
	pub pixels: Vec<u32>
	//should be an array but idk how to
}

impl Texture{

	pub fn load_from_ppm(filepath: &str)->Texture{
		
		let file = fs::read_to_string(filepath).expect("404: File not found");

		let stream = file.lines();


		let mut data:Vec<u32> = vec![];

		for line in stream{
			
			let mut token = line.split_whitespace();

			//let data = token.next().unwrap();

			for w in token{
				if w == "#" { break }
				else if w == "P3" { break }
				else{
					data.push(w.parse().unwrap());
				}
			}
		}
		let width: usize = data[0] as usize;
		let height: usize = data[1] as usize;
		let max_val:u32 = data[2];
		let mut pixels: Vec<u32> = vec![0; width*height];

		for i in 0..(data.len()-2)/3 {
			pixels[i] = util::Color(data[(i*3)+3] as u8, data[(i*3)+4] as u8, data[(i*3)+5] as u8, 1.0).to_u32();
		}

		//print!("Texture loaded!");
		Texture{
			width: width,
			height: height,
			pixels: pixels
		}
	}
}