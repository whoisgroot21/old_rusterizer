use std::fs;
use std::fs::File;
use std::io::Read;
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


	//doesnt work
	pub fn load_from_bmp(filepath: &str)->Texture{

		let mut f = File::open(filepath).expect("404: File not found");
		let metadata = fs::metadata(filepath).expect("unable to read metadata");
    	let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    	f.read(&mut buffer).expect("buffer overflow");

    	let width = u32::from_ne_bytes([buffer[18], buffer[19], buffer[20], buffer[21]]) as usize;
    	let height = u32::from_ne_bytes([buffer[22], buffer[23], buffer[24], buffer[25]]) as usize;

    	let mut pixels = vec![0; width*height];

    	let pixel_start = u32::from_ne_bytes([buffer[10], buffer[11], buffer[12], buffer[13]]) as usize;
    	let bpp = u16::from_ne_bytes([buffer[28], buffer[29]]) as usize;

    	let row_size = (bpp as f32 *width as f32/32.).ceil() as u32 *4;
    	let padding = (width%4) as u32;

    	//pixel array abstraction here

    	//for debugging
    	print!("{:?}", buffer);

    	//print!("Texture Loaded!!");
    	Texture{
			width: width,
			height: height,
			pixels: pixels
		}

    }
}