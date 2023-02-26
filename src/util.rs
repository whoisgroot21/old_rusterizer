#![allow(non_snake_case)]

use std::ops::{Add, Sub};
use std::f32;

//implement generics for vector types

pub struct Color(pub u8, pub u8, pub u8, pub f32);

/*pub struct Color {
	R:u8,
	G:u8,
	B:u8
}
*/

impl Color{
	pub fn to_u32(&self) ->u32{
		let Color(R, G, B, A) = self;
		(256*256*(*R) as u32)+(256*(*G) as u32)+(*B) as u32
	}

	pub fn mix(c1:&Color, c2:&Color, t:f32) -> Color{
		let Color(R1, G1, B1, A1) = c1;
		let Color(R2, G2, B2, A2) = c2;
		//c1+(c2-c1)*t;
		let c3:Color = Color((*R1 as f32+(((*R2) as f32 - (*R1) as f32)*t))as u8,
			(*G1 as f32+(((*G2) as f32- (*G1) as f32)as f32*t))as u8,
			(*B1 as f32+(((*B2) as f32- (*B1) as f32)as f32*t))as u8, 1.0);
		c3
	}

	pub fn swap(c1:&mut Color, c2:&mut Color){
		let R1 = c1.0;
		let R2 = c2.0;
		let G1 = c1.1;
		let G2 = c2.1;
		let B1 = c1.2;
		let B2 = c2.2;
		let A1 = c1.3;
		let A2 = c2.3;
		c1.0 = R2;
		c2.0 = R1;
		c1.1 = G2;
		c2.1 = G1;
		c1.2 = B2;
		c2.2 = B1;
		c1.3 = A2;
		c2.3 = A1;
	}
	pub fn copy(&self)->Color{
		let Color(R, G, B, A) = *self;
		Color(R, G, B, A)
	}
}

/*pub trait Vectorx: Add + Sub + Sized { useless
	fn swap<T:Vectorx>(a:T,  b:T);
	normalize
	dot
	cross
	mag
}*/

pub struct vec2{
	pub x:f32,
	pub y:f32
}

impl vec2{
	
	pub fn swap(a:&mut vec2,  b:&mut vec2){
		let c = vec2{x:a.x, y:a.y};
		a.x = b.x; a.y = b.y;
		b.x = c.x; b.y = c.y;
	}

	pub fn to_vec3(&self) -> vec3{
		vec3{
			x: self.x,
			y: self.y,
			z: 0.0
		}
	}

	//pub fn copy(&self)->vec2{}
}

pub struct vec3 {
	pub x:f32,
	pub y:f32,
	pub z:f32
}

impl Add for &vec3 {
	type Output = vec3;

	fn add(self, other:Self) -> vec3{
		vec3{
			x: self.x+other.x,
			y: self.y+other.y,
			z: self.z+other.z
		}
	}
}

impl Sub for &vec3 {
	type Output = vec3;

	fn sub(self, other:Self) -> vec3{
		vec3{
			x: self.x-other.x,
			y: self.y-other.y,
			z: self.z-other.z
		}
	}
}

impl vec3{

	pub fn to_vec2(&self) -> vec2{
		vec2{
			x: self.x,
			y: self.y
		}
	}

	pub fn swap(a:&mut vec3,  b:&mut vec3){
		let c = vec3{x:a.x, y:a.y, z:a.z};
		a.x = b.x; a.y = b.y; a.z = b.z;
		b.x = c.x; b.y = c.y; b.z = c.z;
	}

	pub fn cross(a:&vec3, b:&vec3) -> vec3{
		let n = vec3{
			x: (a.y*b.z)-(b.y*a.z),
			y: (a.x*b.z)-(b.x*a.z),
			z: (a.x*b.y)-(b.x*a.y)
		};
		n
	}

	pub fn dot(a:&vec3, b:&vec3)->f32{ //or f32
		let dot = a.x*b.x + a.y*b.y + a.z*b.z;
		dot
	}

	pub fn normalize(a:&vec3) -> vec3{
		let mag = vec3::magnitude(&a);
		vec3{
			x: (a.x/mag),
			y: (a.y/mag),
			z: (a.z/mag)
		}
	}

	pub fn magnitude(a:&vec3) -> f32{
		let mag = ((a.x*a.x)+(a.y*a.y)+(a.z*a.z)).sqrt();
		mag
	}

	pub fn to_vec4(&self) -> vec4{
		vec4{
			x: self.x,
			y: self.y,
			z: self.z,
			w: 1.0
		}
	}

	pub fn copy(&self)->vec3{
		vec3{
			x: self.x,
			y: self.y,
			z: self.z
		}
	}
}

pub struct vec4 {
	pub x:f32,
	pub y:f32,
	pub z:f32,
	pub w:f32
}


impl vec4{
	pub fn to_vec3(&self) -> vec3{
		if self.w == 0.0 {
			vec3{
				x: self.x,
				y: self.y,
				z: self.z
			}
		}
		else {
			vec3{
				x: self.x/self.w,
				y: self.y/self.w,
				z: self.z/self.w
			}
		}
	}

	pub fn copy(&self)->vec4{
		vec4{
			x: self.x,
			y: self.y,
			z: self.z,
			w: self.w
		}
	}
}

pub struct matrix4x4 {
	pub data: [[f32; 4]; 4]
}

impl matrix4x4 {
	pub fn matrix_vec_mult(&self, vec:&vec4)->vec4{
		vec4{
			x: self.data[0][0]*vec.x+self.data[0][1]*vec.y+self.data[0][2]*vec.z+self.data[0][3]*vec.w,
			y: self.data[1][0]*vec.x+self.data[1][1]*vec.y+self.data[1][2]*vec.z+self.data[1][3]*vec.w,
			z: self.data[2][0]*vec.x+self.data[2][1]*vec.y+self.data[2][2]*vec.z+self.data[2][3]*vec.w,
			w: self.data[3][0]*vec.x+self.data[3][1]*vec.y+self.data[3][2]*vec.z+self.data[3][3]*vec.w
		}
	}
}



pub struct Vertex {
	pub pos: vec4,
	pub normal: Option<vec4>,
	pub tex_coord: Option<vec4>
	//pub color: Option<Color>	
}

pub fn lerp(x1:f32, y1:f32, x2:f32, y2:f32, x:f32) -> f32{ 
	let slope:f32 = (y2-y1) /(x2-x1);
	
	if (x2-x1) == 0.0 {return y2}

	let y = (slope*(x-x1))+y1;
	y
}