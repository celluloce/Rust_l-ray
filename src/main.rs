extern crate l_ray;
extern crate rand;
extern crate threadpool;

use std::f64::consts::*;
use std::fs::File;
use std::thread::{*, self};
use std::io::*;
// PIに使った

// スレッドに使う
use std::sync::mpsc::*;
use threadpool::ThreadPool;

use rand::random;

use l_ray::source::{vector::*, obj::*};

// 画像の情報
const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const MAX: usize = 255;

// pixelあたりのサンプル数
const SPP: u32 = 1000;

// Threadの数
const WORKS: usize = 10;

fn main() {
	let all = WIDTH * HEIGHT;
	let wid = WIDTH as f64;
	let hei = HEIGHT as f64;

	let up: V = V::new_tri(0.0, 1.0, 0.0);
	// "上"の方向
	let fov: f64 = 30.0 * PI / 180.0;
	// 視野角 ラジアンに直した
	let aspect: f64 = wid / hei;
	// アスペクト比（4:3みたいなやつ）

	// 球体のみ用
	// let eye: V = V::new_sig(5.0);
	// // 目の位置
	// let center: V = V::new_sig(0.0);
	// // 注視点

	// in room用
	let eye = V::new_tri(50.0, 52.0, 295.6);
	let center = eye + V {
		x: 0.0,
		y: -0.042612,
		z: -1.0,
	};

	// 視線の基底（全部垂直な単位ベクトル）
	let WE: V = V::norm(eye - center);
	// 視線の単位ベクトルj
	let UE: V = V::norm(V::cross(up, WE));
	// 視線と"上"の方向に垂直な単位ベクトル
	let VE: V = V::cross(WE, UE);
	// WEとUEに垂直な単位ベクトル（双方正規化されてるため正規化の必要なし）

	// scene 初期化
	//let scene = obj::Scene::new_mul();
	let scene = Scene::in_room();

	let mut push_vec: Vec<JoinHandle<_>>= Vec::with_capacity(WORKS);
	for i in (0..WORKS).rev() {
		let scene = scene.clone();
		let push_th = thread::spawn(move || {
			let mut buf_vec = Vec::with_capacity(WORKS);

			for k in 0..all / WORKS {
				let mut write_push = V::new();
				let w_num = k + all / WORKS * i;

				for j in 0..SPP as usize {
					let x = (w_num % WIDTH) as f64;
					let y = (HEIGHT - (w_num / WIDTH)) as f64;
					let mut ray = Ray::new();

					ray.o = eye;
					ray.d = {
						let tf = (fov * 0.5).tan();
						let rpx = 2.0 * (x + random::<f64>()) / wid - 1.0;
						let rpy = 2.0 * (y + random::<f64>()) / hei - 1.0;
						let w: V = V::norm(V {
							x: aspect * tf * rpx,
							y: tf * rpy,
							z: -1.0,
						});
						UE * V::new_sig(w.x) + VE * V::new_sig(w.y) + WE * V::new_sig(w.z)
					};
					let mut ill_l = V::new_sig(0.0);
					let mut refl_l = V::new_sig(1.0) ;
					for depth in 0..10 {
						let h: Option<Hit> = scene.intersect(&ray, 1e-4, 1e+10);
						if let Some(s) = h {
							ill_l = ill_l + refl_l * s.sphere.ill;
							ray.o = s.p;
							ray.d = {
								let dot_nd = V::dot(s.n, -ray.d);
								if s.sphere.s_type == SurfaceiType::Diffuse {
									let n = if dot_nd > 0.0 { s.n } else { -s.n };
									let (u, v) = n.tangent_space();
									let d: V = {
										let r = random::<f64>().sqrt();
										let t = 2.0 * PI * random::<f64>();
										let x = r * t.cos();
										let y = r * t.sin();
										V {
											x: x,
											y: y,
											z: 0.0_f64.max(1.0 - x * x - y * y).sqrt(),
										}
									};
									u * V::new_sig(d.x) + v * V::new_sig(d.y) + n * V::new_sig(d.z)
								} else if s.sphere.s_type == SurfaceiType::Mirror {
									V::new_sig(2.0 * dot_nd) * s.n + ray.d
								} else if s.sphere.s_type == SurfaceiType::Fresnel {

									let ray = ray.clone();
									fresnel(s, ray)

								} else {
									panic!("dont hit sphere")
								}
							};
							refl_l = refl_l * s.sphere.refl;
						} else {break;}
						if refl_l.x.max(refl_l.y.max(refl_l.z)) == 0.0 {break;}
					}
					write_push = write_push + ill_l / V::new_sig(SPP as f64);
				}
				buf_vec.push(write_push);
				// buf_pushにVのやつをぶちこむ
				if k % 1000 == 0{
					print!("thread {}/{}: ", i, WORKS - 1);
					println!("{}/{} done", k, all / WORKS);
				}
				if k == all/WORKS - 1{
					println!("thread {}/{} done",i, WORKS - 1 );
				}
			}
			buf_vec
			//Thread終わりに返す
		});
		push_vec.push(push_th);
	}
	let tonemap = |v: f64| {
		use std::cmp::*;
		min(max((v.powf(1.0 / 2.2) * 255.0) as u32, 0), 255)
	};

	// ppmファイル生成
	let mut file = File::create("ideal.ppm").unwrap();
	file.write_all(format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, 255).as_bytes())
		.unwrap();

	for i in 0..WORKS {
		let job_write = push_vec.pop().unwrap().join().unwrap();

		for n in job_write.iter() {
			file.write_all(format!("{} {} {}\n",
								   tonemap(n.x.abs()),
								   tonemap(n.y.abs()),
								   tonemap(n.z.abs())).as_bytes()).unwrap();
		}

	}
}

fn fresnel(s: Hit, ray: Ray) -> V{
	let dot_nd = V::dot(s.n, -ray.d);
	let ior = s.sphere.ior;
	let (n, eta)
		= if dot_nd > 0.0 { (s.n, 1.0 / ior) }else { (-s.n, ior)};
	let wt: Option<V> = {
		let t = V::dot(n, -ray.d);
		let t2 = 1.0 - eta * eta * (1.0 - t * t);
		if t2 >= 0.0 {
			Some(V::new_sig(eta) * (n * V::new_sig(t) + ray.d) - n * V::new_sig(t2.sqrt()))
		} else {
			None
		}
	};
	if let Some(wt_s) = wt {
		let fr = {
			let cos = if dot_nd > 0.0 {
				dot_nd
			} else  {
				V::dot(wt_s, s.n)
			};
			let r = (1.0 - ior) / (1.0 + ior);
			r * r + (1.0 - r * r) * (1.0 - cos).powf(5.0)
		};
		if random::<f64>() < fr {
			return V::new_sig(2.0 * dot_nd) * s.n + ray.d;
		} else {
			return wt_s;
		}
	} else {
		return V::new_sig(2.0 * dot_nd) * s.n + ray.d;
	}

}
