mod utils;

use wasm_bindgen::prelude::*;
extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, snake!");
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnakeBlock {
  SquareSize = 100
}


#[wasm_bindgen]
pub struct Snake {
  snake_blocks: Vec<SnakeBlock>,
}

#[wasm_bindgen]
impl Snake {

   pub fn snake_blocks(&self) -> *const SnakeBlock {
    self.snake_blocks.as_ptr()
  }

  pub fn new() -> Snake {
    let snake_blocks = vec![SnakeBlock::SquareSize];

    Snake {
      snake_blocks,
    }
  }
}

#[wasm_bindgen]
pub struct Board {
  width: u32,
  height: u32,
  area: Vec<u32>,
  snake_head_x: u32,
  snake_head_y: u32,
}

#[wasm_bindgen]
impl Board {
  pub fn get_width(&self) -> u32 {
    self.width
  }

  pub fn set_width(&mut self, width: u32) {
    self.width = width;
  }

  pub fn get_height(&self) -> u32 {
    self.height
  }

  pub fn set_height(&mut self, height: u32) {
    self.height = height;
  }

   pub fn snake_head_x(&self) -> u32 {
    self.snake_head_x
  }

   pub fn set_snake_head_x(&mut self, x: u32) {
    self.snake_head_x = x;
  }

  pub fn snake_head_y(&self) -> u32 {
    self.snake_head_y
  }

   pub fn set_snake_head_y(&mut self, y: u32) {
    self.snake_head_y = y;
  }


  pub fn get_area(&self) -> Vec<u32> {
    self.area.clone()
  }

  pub fn new() -> Board {
    utils::set_panic_hook();
    let width = 10;
    let height = 10;
    let area = vec![0; 100];
    let snake_head_x = 5;
    let snake_head_y = 5;

    Board {
      width,
      height,
      area,
      snake_head_x,
      snake_head_y,
    }
  }

   pub fn get_snake_position(&mut self) {
    let position = (self.height * self.snake_head_y) + self.snake_head_x;
    // let mut counter = 1;
    // for element in self.area.iter_mut() {
    //   if counter == position{
    //     *element = 1 as u32;
    //   }
    //   counter += 1;
    // }
    self.area[position as usize] = 1;
    // log!("area {:?}", self.area)
  }

  pub fn increment_snake_x(&mut self) {
    if self.snake_head_x < 9{
        self.snake_head_x += 1;
        self.get_snake_position()
    } else {
      self.snake_head_x = 0;
      self.get_snake_position()
    }
  }

  pub fn decrement_snake_x(&mut self) {
    if self.snake_head_x > 0{
        self.snake_head_x -= 1;
    } else {
      self.snake_head_x = 9;
    }
    self.get_snake_position()
  }

  pub fn increment_snake_y(&mut self) {
    if self.snake_head_y < 9{
        self.snake_head_y += 1;
    } else {
      self.snake_head_y = 0;
    }
    self.get_snake_position()
  }

   pub fn decrement_snake_y(&mut self) {
    if self.snake_head_y > 0{
        self.snake_head_y -= 1;
    } else {
      self.snake_head_y = 9;
    }
    self.get_snake_position()
  }
}

