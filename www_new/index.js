import { Board, Snake, Food } from "snake"

const board = Board.new()
const snake = Snake.new()
const food = Food.new()
const height = board.get_height()
const width = board.get_width()
const snakeBlockSize = 100
const foodSize = 20
const speed = 150
let interval

const getRandomInt = (max) => Math.floor(Math.random() * Math.floor(max))

const randomInt = getRandomInt(90)
food.check_new_food_position(randomInt, board)
board.add_food_to_area(food.get_numeric_position())

const canvas = document.getElementById("snake-canvas")
canvas.height = (snakeBlockSize) * height
canvas.width = (snakeBlockSize) * width

const ctx = canvas.getContext('2d')
ctx.fillRect(board.snake_head_x() * snakeBlockSize, board.snake_head_y() * snakeBlockSize, snakeBlockSize, snakeBlockSize)

const scoreHeader = document.getElementById("score")
scoreHeader.innerText = 'Score: 0'

const clearRect = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
}

const drawRect = () => {
  board.get_body_x_positions().forEach((x, index) => {
    const y = board.get_body_y_positions()[index]
    ctx.fillRect(x - 1, y - 1, snakeBlockSize + 2, snakeBlockSize + 2)
  });
}

const drawFood = () => {
  const x = food.get_coordinate_position()[0]
  const y = food.get_coordinate_position()[1]
  ctx.fillRect(x, y, foodSize, foodSize)
}

const snakeDirection = (direction) => {
  switch (direction) {
    case 'w':
      return board.decrement_snake_y(snake, food)
    case 'a':
      return board.decrement_snake_x(snake, food)
    case 's':
      return board.increment_snake_y(snake, food)
    case 'd':
      return board.increment_snake_x(snake, food)
    default: return null
  }
}

const renderScore = () => {
  const scoreHeader = document.getElementById("score")
  const score = (snake.get_snake_length() - 1) * 10
  scoreHeader.innerText = `Score: ${score}`
}

const movesSnake = (direction) => {
  clearRect()
  drawRect()
  drawFood()
  clearInterval(interval)
  interval = setInterval(() => {
    clearRect()
    snakeDirection(direction)
    if(!food.get_is_eaten()){
      drawFood()
    } else {
      let isFree
      food.set_is_eaten()
      const newRandomInt = getRandomInt(90)
      isFree = food.check_new_food_position(newRandomInt, board)
      while (!isFree) {
        const newRandomInt = getRandomInt(90)
        isFree = food.check_new_food_position(newRandomInt, board)
      }
      board.add_food_to_area(food.get_numeric_position())
      drawFood()
    }
    renderScore()
    drawRect()
  }, speed)
}

document.addEventListener('keydown', function(event) {
  if(snake.get_is_alive()){
    const key = event.key
    if(key === 'w'){
      movesSnake(key)
    } else if(key === 'a'){
      movesSnake(key)
    } else if(key === 's'){
      movesSnake(key)
    } else if(key === 'd'){
      movesSnake(key)
    }
  } else {
    const scoreHeader = document.getElementById("score")
    scoreHeader.innerText = 'Game Over'
    clearInterval(interval)
  }
})

const startButton = document.getElementById("start")
startButton.addEventListener('click', () => {
  interval = setInterval(() => {
    if(snake.get_is_alive){
      clearRect()
      board.increment_snake_x(snake, food)
      drawRect()
      drawFood()
    } else {
      const scoreHeader = document.getElementById("score")
      scoreHeader.innerText = 'Game Over'
      clearInterval(interval)
    }
  }, speed)
})