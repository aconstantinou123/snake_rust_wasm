mod utils;

use wasm_bindgen::prelude::*;
extern crate web_sys;

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

#[wasm_bindgen]
pub struct Food {
  is_eaten: bool,
  numeric_position: u32,
  coordinate_position: Vec<u32>,
}

#[wasm_bindgen]
impl Food {
  pub fn new() -> Food {
    let is_eaten = false;
    let numeric_position = 0;
    let y = 0;
    let x = 0;
    let coordinate_position = vec![x, y];

    Food {
      is_eaten,
      numeric_position,
      coordinate_position,
    }
  }

  pub fn get_is_eaten(&self) -> bool {
    self.is_eaten
  }

  pub fn set_is_eaten(&mut self) {
    self.is_eaten = !self.is_eaten;
  }

  pub fn get_numeric_position(&self) -> u32 {
    self.numeric_position
  }

  pub fn get_coordinate_position(&self) -> Vec<u32> {
    self.coordinate_position.clone()
  }

  pub fn check_new_food_position(&mut self, position: u32, board: &mut Board) -> bool {
    self.numeric_position = position;
    let y = ((self.numeric_position as f64 / 10.0).floor() * 100.0 + 50.0) as u32;
    let x = (self.numeric_position % 10) * 100 + 50;
    self.coordinate_position = vec![x, y];
    if board.position_is_empty(position) == true { true } else { false }
  }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Snake {
  snake_blocks: Vec<u32>,
  is_alive: bool,
}

#[wasm_bindgen]
impl Snake {

   pub fn snake_blocks(&self) -> Vec<u32> {
    self.snake_blocks.clone()
  }

  pub fn get_snake_length(&self) -> usize {
    self.snake_blocks.len()
  }

  pub fn add_snake_block(&mut self) {
    self.snake_blocks.push(100);
  }

  pub fn get_is_alive(&self) -> bool {
    self.is_alive
  }

  pub fn set_is_alive(&mut self) {
    self.is_alive = false
  }

  pub fn new() -> Snake {
    let snake_blocks = vec![100];
    let is_alive = true;

    Snake {
      snake_blocks,
      is_alive,
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
  body_x_positions: Vec<u32>,
  body_y_positions: Vec<u32>,
}

#[wasm_bindgen]
impl Board {

  pub fn get_body_x_positions(&self) -> Vec<u32> {
    self.body_x_positions.clone()
  }

   pub fn get_body_y_positions(&self) -> Vec<u32> {
    self.body_y_positions.clone()
  }

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
    let body_x_positions = vec![500];
    let body_y_positions = vec![500];

    Board {
      width,
      height,
      area,
      snake_head_x,
      snake_head_y,
      body_x_positions,
      body_y_positions,
    }
  }

  pub fn position_is_empty(&self, position: u32) -> bool {
    if self.area[position as usize] == 1 { false } else { true }
  } 

  pub fn add_food_to_area(&mut self, position: u32) {
    self.area[position as usize] = 2;
  }

  pub fn add_snake_block(&mut self, snake: &mut Snake){
    snake.add_snake_block();
    if self.body_x_positions.len() == 0 {
      self.body_x_positions.push(self.snake_head_x - 100);
      self.body_y_positions.push(self.snake_head_y);
    } else {
      let last_x = self.body_x_positions.last().cloned().unwrap();
      let last_y = self.body_y_positions.last().cloned().unwrap();
      self.body_x_positions.push(last_x - 100);
      self.body_y_positions.push(last_y);
    }
  }

   pub fn get_snake_position(&mut self, snake: &Snake) {
    let mut prev_x_value = 0;
    let mut prev_y_value = 0;
    let mut temp_x = 0;
    let mut temp_y = 0;
    // self.area = vec![0; 100];
    for element in self.area.iter_mut() {
      if *element == 1 {
        *element = 0
      }
    }
    for (index, _element) in snake.snake_blocks().iter().enumerate() {
      if index == 0 {
        prev_x_value = self.body_x_positions[index];
        prev_y_value = self.body_y_positions[index];
        self.body_x_positions[index as usize] = self.width * self.snake_head_x * 10;
        self.body_y_positions[index as usize] = self.height * self.snake_head_y * 10;
      } else {
        temp_x = prev_x_value;
        temp_y = prev_y_value;
        prev_x_value = self.body_x_positions[index];
        prev_y_value = self.body_y_positions[index];
        self.body_x_positions[index as usize] = temp_x;
        self.body_y_positions[index as usize] = temp_y;
      }
      let position = (self.body_y_positions[index as usize] / 10) + (self.body_x_positions[index as usize] / 100);
      self.area[position as usize] = 1;
    }
  }

  pub fn detect_collision(&mut self, snake: &mut Snake, food: &mut Food) {
    let position = (self.height * self.snake_head_y) + self.snake_head_x;
    if self.area[position as usize] == 1 {
        snake.set_is_alive()
    } else if self.area[position as usize] == 2 {
        self.add_snake_block(snake);
        food.set_is_eaten();
    }
  }
  
  pub fn increment_snake_x(&mut self, snake: &mut Snake, food: &mut Food) {
    if self.snake_head_x < 9{
        self.snake_head_x += 1;
    } else {
      self.snake_head_x = 0;
    }
    self.detect_collision(snake, food);
    self.get_snake_position(snake)
  }

  pub fn decrement_snake_x(&mut self, snake: &mut Snake, food: &mut Food) {
    if self.snake_head_x > 0{
        self.snake_head_x -= 1;
    } else {
      self.snake_head_x = 9;
    }
    self.detect_collision(snake, food);
    self.get_snake_position(snake)
  }

  pub fn increment_snake_y(&mut self, snake: &mut Snake, food: &mut Food) {
    if self.snake_head_y < 9{
        self.snake_head_y += 1;
    } else {
      self.snake_head_y = 0;
    }
    self.detect_collision(snake, food);
    self.get_snake_position(snake)
  }

   pub fn decrement_snake_y(&mut self, snake: &mut Snake, food: &mut Food) {
    if self.snake_head_y > 0{
        self.snake_head_y -= 1;
    } else {
      self.snake_head_y = 9;
    }
    self.detect_collision(snake, food);
    self.get_snake_position(snake)
  }
}

