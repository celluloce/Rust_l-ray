extern crate l_ray;

use std::fs::File;
use std::io::*;

use l_ray::source::{vector::*, *};
use std::f64::consts::*;
// PIに使った

// 画像の情報
const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const MAX: usize = 255;

fn main() {
    let up: V = V::new_tri(0.0, 1.0, 0.0);
    // "上"の方向
    let fov: f64 = 30.0 * PI / 180.0;
    // 視野角 ラジアンに直した
    let aspect: f64 = WIDTH as f64 / HEIGHT as f64;
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

    // let scene = obj::Scene::new_mul();
    let scene = obj::Scene::in_room();

    let mut file = File::create("ideal.ppm").unwrap();
    file.write_all(format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, MAX).as_bytes())
        .unwrap();

    for i in 0..HEIGHT * WIDTH {
        let x = (i % WIDTH) as f64;
        let y = HEIGHT as f64 - (i / WIDTH) as f64;

        let mut ray = obj::Ray::new();

        ray.o = eye;
        // 目の位置
        {
            let tf = (fov * 0.5).tan();
            let rpx = 2.0 * x / WIDTH as f64 - 1.0;
            let rpy = 2.0 * y / HEIGHT as f64 - 1.0;
            let w: V = V::norm(V {
                x: aspect * tf * rpx,
                y: tf * rpy,
                z: -1.0,
            });
            ray.d = UE * V::new_sig(w.x) + VE * V::new_sig(w.y) + WE * V::new_sig(w.z);
        }
        // 目線の長さ（まだよくわかってない）

        let h: Option<obj::Hit> = scene.intersect(&ray, 0.0, 1e+10);

        let tonemap = |v: f64| {
            use std::cmp::*;
            min(max((v.powf(1.0 / 2.2) * 255.0) as u32, 0), 255)
            // ちゃんとガンマ補正したよ
        };

        if let Some(s) = h {
            // 反射率
            let c: vector::V = s.sphere.refl * V::new_sig(V::dot(s.n, -ray.d));
            // V::new_sig(V::dot(s.n, -ray.d))を
            // 入れるとランバート反射になる。
            file.write_all(
                format!(
                    "{} {} {}\n",
                    tonemap(c.x.abs()),
                    tonemap(c.y.abs()),
                    tonemap(c.z.abs())
                ).as_bytes(),
            );

        // // 球面上の接点の法線
        // let n: vector::V = s.n;
        // file.write_all(
        //     format!(
        //         "{} {} {}\n",
        //         tonemap(n.x.abs()),
        //         tonemap(n.y.abs()),
        //         tonemap(n.z.abs())
        //     ).as_bytes(),
        // );
        } else {
            file.write_all(format!("{} {} {}\n", 0, 0, 0).as_bytes());
        }
    }

    // for c in 0..HEIGHT {
    //     for d in 0..WIDTH {
    //         let red = c / 2;
    //         let green = d / 2;
    //         file.write_all(format!("{} {} {}\n", red, green, 255).as_bytes());
    //     }
    // }
}
