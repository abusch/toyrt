use cg::{self, prelude::*};

use crate::{Matrix4f, Point3f, Vec3f};

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub m: Matrix4f,
    pub m_inv: Matrix4f,
}

impl Transform {
    pub fn new(m: Matrix4f) -> Self {
        Transform {
            m_inv: m.invert().unwrap(),
            m,
        }
    }

    pub fn transform_point(&self, p: Point3f) -> Point3f {
        cg::Transform::transform_point(&self.m, p)
    }

    pub fn transform_vec(&self, v: Vec3f) -> Vec3f {
        cg::Transform::transform_vector(&self.m, v)
    }

    pub fn transform_normal(&self, n: Vec3f) -> Vec3f {
        let m = self.m_inv;
        let (x, y, z) = (n.x, n.y, n.z);

        Vec3f::new(
            m[0][0] * x + m[1][0] * y + m[2][0] * z,
            m[0][1] * x + m[1][1] * y + m[2][1] * z,
            m[0][2] * x + m[1][2] * y + m[2][2] * z,
        )
    }

    pub fn invert(&self) -> Self {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }
}

impl From<Matrix4f> for Transform {
    fn from(m: Matrix4f) -> Self {
        Transform::new(m)
    }
}
