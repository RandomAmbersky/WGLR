#[repr(C)]
pub struct Vertex2D {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2],
}

pub trait AsF32Slice {
    fn as_f32_slice<'a>(&'a self) -> &'a [f32];
}

impl AsF32Slice for [Vertex2D] {
    fn as_f32_slice<'a>(&'a self) -> &'a [f32] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_ptr() as *const f32,
                self.len() * 4,
            )
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn as_slice() {
        let vertices = [
            Vertex2D {
                pos: [12.0, 15.0],
                tex_coords: [22.2, 31.1],
            },
            Vertex2D {
                pos: [25.3, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex2D {
                pos: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex2D {
                pos: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex2D {
                pos: [0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex2D {
                pos: [1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
        ];
        let slice = vertices.as_f32_slice();
        assert_eq!(slice[0], 12.0);
        assert_eq!(slice[1], 15.0);
        assert_eq!(slice[2], 22.2);
        assert_eq!(slice[3], 31.1);
        assert_eq!(slice[4], 25.3);
    }
}
