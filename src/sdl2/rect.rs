#[deriving(Clone)]
pub struct Rect {
    x: i32, y: i32,
    w: i32, h: i32
}

#[deriving(Clone)]
pub struct Point {
    x: i32, y: i32
}

pub mod ffi {
    use std::libc::c_int;

    use rect::Rect;
    use rect::Point;

    type SDL_bool = c_int;

    type SDL_Rect = Rect;
    type SDL_Point = Point;

    externfn!(fn SDL_HasIntersection(a: *SDL_Rect, b: *SDL_Rect) -> SDL_bool)
    externfn!(fn SDL_IntersectRect(a: *SDL_Rect, b: *SDL_Rect, 
                                   res: *SDL_Rect) -> SDL_bool) 
    externfn!(fn SDL_IntersectRectAndLine(a: *SDL_Rect, 
                                         x1: *c_int, y1: *c_int, 
                                         x2: *c_int, y2: *c_int) -> SDL_bool)
    externfn!(fn SDL_UnionRect(a: *SDL_Rect, b: *SDL_Rect, 
                               res: *SDL_Rect))
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x: x, y: y, w: w, h: h }
    }

    pub fn has_intersection(&self, other: &Rect) -> bool {
        unsafe {
            ffi::SDL_HasIntersection(self, other) == 1
        }
    }

    pub fn intersect_rect(&self, other: &Rect) -> Option<Rect> {
        let out: Rect = Rect::new(0, 0, 0, 0);

        let result = unsafe {
            ffi::SDL_IntersectRect(self, other, &out) == 1
        };

        match result {
            true => Some(out),
            _ => None
        }
    }

    pub fn intersect_line(&self, start: &Point, end: &Point) -> Option<(Point, Point)> {
        let out_start: Point = start.clone();
        let out_end: Point = end.clone();

        let res = unsafe {
            ffi::SDL_IntersectRectAndLine(self, 
                                          &out_start.x, &out_start.y, 
                                          &out_end.x, &out_end.y) == 1
        };

        match res {
            true => Some((out_start, out_end)),
            _ => None
        }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let out: Rect = Rect::new(0, 0, 0, 0);
        unsafe {
            ffi::SDL_UnionRect(self, other, &out);
        }

        out
    }
}

