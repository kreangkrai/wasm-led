use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point{
    pub x:i32,
    pub y:i32,
}

#[wasm_bindgen]
impl Point{
    #[wasm_bindgen(constructor)]
    pub fn new(x:i32,y:i32)->Self{
        Point{x:x,y:y}
    }
    pub fn get(&self) -> Point{
        Point {x:self.x,y:self.y}
    }
}
