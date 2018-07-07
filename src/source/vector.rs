use std::ops::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct V {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V {
    // constructor
    pub fn new() -> V {
        V {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_sig(v: f64) -> V {
        V { x: v, y: v, z: v }
    }

    pub fn new_tri(x: f64, y: f64, z: f64) -> V {
        V { x: x, y: y, z: z }
    }
}

impl Add for V {
    type Output = V;

    fn add(self, v: V) -> V {
        V {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}
impl Sub for V {
    type Output = V;

    fn sub(self, v: V) -> V {
        V {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}
impl Mul for V {
    type Output = V;

    fn mul(self, v: V) -> V {
        V {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}
impl Div for V {
    type Output = V;

    fn div(self, v: V) -> V {
        V {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}
impl Neg for V {
    type Output = V;

    fn neg(self) -> V {
        V {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl V {
    // math function
    pub fn dot(self, v: V) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: V) -> V {
        V {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn norm(self) -> V {
        V {
            x: self.x / self.dot(self).sqrt(),
            y: self.y / self.dot(self).sqrt(),
            z: self.z / self.dot(self).sqrt(),
        }
    }
	pub fn tangent_space(self) -> (V, V){
		// 一つのベクトルを元に、直交の単位ベクトルを生成する関数。
		// 外積を使うことで同じ事はできるが、こちらの方が速度で勝っている。
		// 理解が難しいので今後の課題
		let s = if self.z >= 0.0 { 1.0 } else { -1.0 };

		let a = -1.0 / (s + self.z);
		let b = self.x * self.y * a;

		// return
		(
			V {
				x: 1.0 + s * self.x * self.x * a,
				y: s * b,
				z: -s * self.x,
			},
			V {
				x: b,
				y: s + self.y * self.y * a,
				z: -self.y,
			}
		)
	}
}
