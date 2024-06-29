import { applyAB, parseBoard, printBoard, tickBoard } from "./lib"

// const board = {
//   tick: 0,
//   cells: new Map<string, Cell>([
//     [JSON.stringify({ x: 0, y: 0 }), { type: 'Int', value: 1 }],
//     [JSON.stringify({ x: 10, y: 10 }), { type: 'BinOp', op: '+' }],
//   ])
// }

// console.log(printBoard(board))

`
1 > . < 1
`

const parsed = parseBoard(
`1 > . < 2`)

console.log(printBoard(parsed))

var history = [parsed]
var current = applyAB(parsed, 6, 0)

console.log(current.tick)
console.log(printBoard(current))
for (let index = 0; index < 40; index++) {
  const {board: next, outputs} = tickBoard(current, history)
  history.push(next)
  current = next
  console.log('---')
  console.log(current.tick)
  console.log(index)
  console.log(printBoard(current))
  if (outputs.length > 0) {
    console.log(outputs)
    break;
  }
}


// console.log(current)
// console.log(outputs)
`
. . . 1 . . . . . .
. . A - . > . > . .
. . . . . 0 = 6 @ 1
. . A * . . . . 3 .
. . < . . A + S . .
-4 @ 1 v . . . . . .
. 3 . . . . . . . .
. . 1 @ 4 . . . . .
. . . 3 . . . . . .
`