use crate::util::*;
use crate::WindowHandler;

impl WindowHandler{
		
	pub fn draw_line(&mut self, x1:u16, y1:u16, x2:u16, y2:u16){
		let diffx = if x2>x1 {x2-x1+1} else {x1-x2+1};
		let diffy = if y2>y1 {y2-y1+1} else {y1-y2+1};

		if diffx > diffy{
			for i in 0..diffx{
				let x3 = if x2>x1 {x1+i} else {x1-i};
				let y3 = lerp(x1 as f32, y1 as f32, x2 as f32, y2 as f32, x3 as f32) as u16;
				self.putPixel(x3, y3);
			}
		}
		else{
			for i in 0..diffy{
				let y3 = if y2>y1 {y1+i} else {y1-i};
				let x3 = lerp(y1 as f32, x1 as f32, y2 as f32, x2 as f32, y3 as f32) as u16;
				self.putPixel(x3, y3);
			}
		}
	}

	pub fn draw_line_with_vec(&mut self, p1:&vec3, p2:&vec3){
		let diffx = if p2.x>p1.x {p2.x-p1.x+1.} else {p1.x-p2.x+1.};
		let diffy = if p2.y>p1.y {p2.y-p1.y+1.} else {p1.y-p2.y+1.};
	
		if diffx > diffy{
			for i in 0..diffx as u32{
				let i = i as f32;
				let x3 = (if p2.x>p1.x {p1.x+i} else {p1.x-i}) as u16;
				let y3 = lerp(p1.x, p1.y, p2.x, p2.y, x3 as f32) as u16;
				let z3 = lerp(p1.x, p1.z, p2.x, p2.z, x3 as f32);
				if z3 <= self.depth_buffer[x3 as usize + (y3 as usize*self.win_WIDTH)]{
					self.depth_buffer[x3 as usize + (y3 as usize*self.win_WIDTH)] = z3;
					self.putPixel(x3, y3);
				}
			}
		}
		else{
			for i in 0..diffy as u32{
				let i = i as f32;
				let y3 = (if p2.y>p1.y {p1.y+i} else {p1.y-i}) as u16;
				let x3 = lerp(p1.y, p1.x, p2.y, p2.x, y3 as f32) as u16;
				let z3 = lerp(p1.y, p1.z, p2.y, p2.z, y3 as f32);
				if z3 < self.depth_buffer[x3 as usize + (y3 as usize*self.win_WIDTH)]{
					self.depth_buffer[x3 as usize + (y3 as usize*self.win_WIDTH)] = z3;
					self.putPixel(x3, y3);
				}
			}
		}
	}

	
	pub fn draw_triangle(&mut self, style:&str, p1: &vec3, p2: &vec3, p3: &vec3){
		if style == "WIREFRAME"{
			self.draw_line(p1.x as u16, p1.y as u16, p2.x as u16, p2.y as u16);
			self.draw_line(p2.x as u16, p2.y as u16, p3.x as u16, p3.y as u16);
			self.draw_line(p1.x as u16, p1.y as u16, p3.x as u16, p3.y as u16);
		}
		else if style == "FILL"{
			let mut sp1 = p1.copy();
			let mut sp2 = p2.copy();
			let mut sp3 = p3.copy();
			if sp1.y > sp2.y{vec3::swap(&mut sp1, &mut sp2);}
			if sp1.y > sp3.y{vec3::swap(&mut sp1, &mut sp3);}
			if sp2.y > sp3.y{vec3::swap(&mut sp2, &mut sp3);}	
			if sp1.y == sp2.y && sp1.x>sp2.x{vec3::swap(&mut sp1, &mut sp2);}	
			for i in 0..(sp2.y-sp1.y+1.0) as u16{
				let i = f32::from(i);
				//lerp line 1-2
				let x1 = lerp(sp1.y, sp1.x, sp2.y, sp2.x, sp1.y+i).floor();
				let z1 = lerp(sp1.y, sp1.z, sp2.y, sp2.z, sp1.y+i);
				//lerp line 1-3
				let x2 = lerp(sp1.y, sp1.x, sp3.y, sp3.x, sp1.y+i).ceil();
				let z2 = lerp(sp1.y, sp1.z, sp3.y, sp3.z, sp1.y+i);
				//draw
				self.draw_line_with_vec(&vec3{x: x1, y:(sp1.y+i), z: z1}, &vec3{x: x2, y:(sp1.y+i), z: z2});
			}
			for i in 0..(sp3.y-sp2.y+1.0) as u16{
				let i = f32::from(i);
				//lerp line 1-2
				let x1 = lerp(sp2.y, sp2.x, sp3.y, sp3.x, sp2.y+i).floor();
				let z1 = lerp(sp2.y, sp2.z, sp3.y, sp3.z, sp2.y+i);
				//lerp line 1-3
				let x2 = lerp(sp1.y, sp1.x, sp3.y, sp3.x, sp2.y+i).ceil();
				let z2 = lerp(sp1.y, sp1.z, sp3.y, sp3.z, sp2.y+i);
				//draw
				self.draw_line_with_vec(&vec3{x: x1, y:(sp2.y+i), z: z1}, &vec3{x: x2, y:(sp2.y+i), z: z2});
			}
		}
		else {panic!("Invalid style");}
	}
}
