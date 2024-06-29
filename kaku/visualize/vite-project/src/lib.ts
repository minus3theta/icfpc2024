export type IntCell = {
  type: 'Int',
  value: number
}

export type LShiftCell = {
  type: 'LShift',
}

export type RShiftCell = {
  type: 'RShift',
}

export type UShiftCell = {
  type: 'UShift',
}

export type DShiftCell = {
  type: 'DShift',
}

export type BinOp = '+' | '-' | '*' | '/' | '%' | '=' | '#'

export type BinOpCell = {
  type: 'BinOp',
  op: BinOp
}

export type OutputCell = {
  type: 'Output',
}

export type ACell = {
  type: 'A',
}

export type BCell = {
  type: 'B',
}

export type TimeCell = {
  type: 'Time'
}

export type EmptyCell = {
  type: 'Empty'
}

export type Cell = IntCell | LShiftCell | RShiftCell | UShiftCell | DShiftCell | BinOpCell | OutputCell | ACell | BCell | TimeCell | EmptyCell

export const printCell = (cell: Cell): string => {
  switch (cell.type) {
    case 'Int':
      return cell.value.toString()
    case 'LShift':
      return '<'
    case 'RShift':
      return '>'
    case 'UShift':
      return '^'
    case 'DShift':
      return 'v'
    case 'BinOp':
      return cell.op
    case 'Output':
      return 'S'
    case 'A':
      return 'A'
    case 'B':
      return 'B'
    case 'Time':
      return '@'
    case 'Empty':
      return '.'
  }
}

export type Point = {
  x: number,
  y: number
}

export type Board = {
  tick: number,
  cells: Map<string, Cell>
}

export const printBoard = (board: Board): string => {
  const { cells } = board
  const xs = Array.from(cells.keys()).map(p => JSON.parse(p).x)
  const ys = Array.from(cells.keys()).map(p => JSON.parse(p).y)
  const minX = Math.min(...xs)
  const maxX = Math.max(...xs)
  const minY = Math.min(...ys)
  const maxY = Math.max(...ys)

  var boardStr = ''

  for (let y = minY; y <= maxY; y++) {
    for (let x = minX; x <= maxX; x++) {
      if (x != minX) {
        boardStr += ' '
      }
      const cell = cells.get(JSON.stringify({ x, y })) || { type: 'Empty' }
      boardStr += printCell(cell)
    }
    boardStr += '\n'
  }
  return boardStr
}

export const parseBoard = (boardStr: string): Board => {
  const lines = boardStr.split('\n')
  const cells = new Map<string, Cell>()
  lines.forEach((line, y) => {
    line.split(' ').forEach((cellStr, x) => {
      const cell: Cell = cellStr === '<' ? { type: 'LShift' } :
        cellStr === '>' ? { type: 'RShift' } :
          cellStr === '^' ? { type: 'UShift' } :
            cellStr === 'v' ? { type: 'DShift' } :
              cellStr === '+' ? { type: 'BinOp', op: '+' } :
                cellStr === '-' ? { type: 'BinOp', op: '-' } :
                  cellStr === '*' ? { type: 'BinOp', op: '*' } :
                    cellStr === '/' ? { type: 'BinOp', op: '/' } :
                      cellStr === '%' ? { type: 'BinOp', op: '%' } :
                        cellStr === '=' ? { type: 'BinOp', op: '=' } :
                          cellStr === '#' ? { type: 'BinOp', op: '#' } :
                            cellStr === 'S' ? { type: 'Output' } :
                              cellStr === 'A' ? { type: 'A' } :
                                cellStr === 'B' ? { type: 'B' } :
                                  cellStr === '@' ? { type: 'Time' } :
                                    cellStr === '.' ? { type: 'Empty' } :
                                      { type: 'Int', value: parseInt(cellStr) }
      if (cell.type !== 'Empty' && !(cell.type === 'Int' && isNaN(cell.value))) {
        cells.set(JSON.stringify({ x, y }), cell)
      }
    })
  })
  return { tick: 1, cells }
}

export const tickBinOpCell = (p: Point, op: BinOp, a: Cell, b: Cell): {add: {p: Point, c: Cell}[], delete: Point[], time: number} => {
  switch (op) {
    case '+':
      if (a.type === 'Int' && b.type === 'Int') {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: { type: 'Int', value: a.value + b.value }},
          {p: {x: p.x, y: p.y+1}, c: { type: 'Int', value: a.value + b.value }}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
    case '-':
      if (a.type === 'Int' && b.type === 'Int') {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: { type: 'Int', value: a.value - b.value }},
          {p: {x: p.x, y: p.y+1}, c: { type: 'Int', value: a.value - b.value }}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
    case '*':
      if (a.type === 'Int' && b.type === 'Int') {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: { type: 'Int', value: a.value * b.value }},
          {p: {x: p.x, y: p.y+1}, c: { type: 'Int', value: a.value * b.value }}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
    case '/':
      if (a.type === 'Int' && b.type === 'Int') {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: { type: 'Int', value: Math.floor(a.value / b.value) }},
          {p: {x: p.x, y: p.y+1}, c: { type: 'Int', value: Math.floor(a.value / b.value) }}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
    case '%':
      if (a.type === 'Int' && b.type === 'Int') {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: { type: 'Int', value: a.value % b.value }},
          {p: {x: p.x, y: p.y+1}, c: { type: 'Int', value: a.value % b.value }}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
    case '=':
      if (equalCell(a, b)) {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: b},
          {p: {x: p.x, y: p.y+1}, c: a}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
    case '#':
      if (!equalCell(a, b)) {
        return {add: [
          {p: {x: p.x+1, y: p.y}, c: b},
          {p: {x: p.x, y: p.y+1}, c: a}
        ], delete: [{x: p.x-1, y: p.y}, {x: p.x, y: p.y-1}], time: 0}
      }
      break;
  }
  return {add: [], delete: [], time: 0}
}

export const tickTimeCell = (p: Point, a: Cell, b: Cell, c: Cell, d: Cell): {add: {p: Point, c: Cell}[], delete: Point[], time: number} => {
  if (a.type === 'Int' && c.type === 'Int' && d.type === 'Int') {
    return {add: [
      {p: {x: p.x - a.value, y: p.y - c.value}, c: b}
    ], delete: [], time: d.value}
  }
  return {add: [], delete: [], time: 0}
}


export const tickCell = (board: Board, point: Point): {add: {p: Point, c: Cell}[], delete: Point[], time: number} => {
  const cell = board.cells.get(JSON.stringify(point))
  if (cell === undefined) {
    return {add: [], delete: [], time: 0}
  }
  switch (cell.type) {
    case 'Int':
      return {add: [], delete: [], time: 0}
    case 'LShift': {
      // if cell exists to the right, delete it and add a new one to the left
      const a = board.cells.get(JSON.stringify({ x: point.x + 1, y: point.y }))
      if (a !== undefined) {
        return {add: [{p: {x: point.x - 1, y: point.y}, c: a}], delete: [{x: point.x + 1, y: point.y}], time: 0}
      }
      break;
    }
    case 'RShift': {
      const a = board.cells.get(JSON.stringify({ x: point.x - 1, y: point.y }))
      if (a !== undefined) {
        return {add: [{p: {x: point.x + 1, y: point.y}, c: a}], delete: [{x: point.x - 1, y: point.y}], time: 0}
      }
      break;
    }
    case 'UShift': {
      const a = board.cells.get(JSON.stringify({ x: point.x, y: point.y + 1 }))
      if (a !== undefined) {
        return {add: [{p: {x: point.x, y: point.y - 1}, c: a}], delete: [{x: point.x, y: point.y + 1}], time: 0}
      }
      break;
    }
    case 'DShift': {
      const a = board.cells.get(JSON.stringify({ x: point.x, y: point.y - 1 }))
      if (a !== undefined) {
        return {add: [{p: {x: point.x, y: point.y + 1}, c: a}], delete: [{x: point.x, y: point.y - 1}], time: 0}
      }
      break;
    }
    case 'BinOp': {
      const a = board.cells.get(JSON.stringify({ x: point.x - 1, y: point.y }))
      const b = board.cells.get(JSON.stringify({ x: point.x, y: point.y - 1 }))
      if (a === undefined || b === undefined) {
        break
      }
      return tickBinOpCell(point, cell.op, a, b)
    }
    case 'Output':
      return {add: [], delete: [], time: 0}
    case 'A':
      throw new Error("A cell should not be present in the board");
    case 'B':
      throw new Error("B cell should not be present in the board");
    case 'Time':
      const a = board.cells.get(JSON.stringify({ x: point.x - 1, y: point.y }))
      const b = board.cells.get(JSON.stringify({ x: point.x, y: point.y - 1 }))
      const c = board.cells.get(JSON.stringify({ x: point.x + 1, y: point.y }))
      const d = board.cells.get(JSON.stringify({ x: point.x, y: point.y + 1 }))
      if (a === undefined || b === undefined || c === undefined || d === undefined) {
        break
      }
      return tickTimeCell(point, a, b, c, d)
    default:
      return {add: [], delete: [], time: 0}
  }
  return {add: [], delete: [], time: 0}
}

export const equalCell = (a: Cell, b: Cell): boolean => {
  if (a.type !== b.type) {
    return false
  }
  switch (a.type) {
    case 'Int':
      return a.value === (b as IntCell).value
    case 'BinOp':
      return a.op === (b as BinOpCell).op
    default:
      return true
  }
}

export const tickBoard = (board: Board, histories: Board[]): {board: Board, outputs: Cell[]} => {
  const { cells } = board
  const newCells = new Map<string, Cell>()
  var deleted: Point[] = []
  var times: Map<number, {p: Point, c: Cell}[]> = new Map()
  var outputs: Cell[] = []
  cells.forEach((_, pointStr) => {
    const point = JSON.parse(pointStr)
    const { add, delete: ps, time } = tickCell(board, point)
    if (time > 0) {
      var s = times.get(time) || []
      add.forEach(newCell => {
        s.push(newCell)
      })
      times.set(time, s)
    }
    deleted = deleted.concat(ps)
    add.forEach((newCell) => {
      const x = newCells.get(JSON.stringify({x: newCell.p.x, y: newCell.p.y}))
      if (x !== undefined) {
        if (!equalCell(x, newCell.c)) {
          throw new Error(`Cell conflict: ${x} vs ${newCell.c} at (${newCell.p.x}, ${newCell.p.y}) on tick ${board.tick + 1}`);
        }
      }
      const target = cells.get(JSON.stringify({ x: newCell.p.x, y: newCell.p.y }))
      if (target !== undefined && target.type === 'Output') {
        outputs.push(newCell.c)
      }
      newCells.set(JSON.stringify({ x: newCell.p.x, y: newCell.p.y }), newCell.c)
    })
  })

  // when time warp
  if (times.size > 0) {
    const timeKeys = Array.from(times.keys())
    if (timeKeys.length > 1) {
      throw new Error(`Multiple time warps at the same time: ${timeKeys}`);
    }
    const time = timeKeys[0]
    if (time > histories.length) {
      throw new Error(`Time warp to a time that does not exist: ${time}`);
    }
    const newBoard = {tick: board.tick - time, cells: new Map(findHistory(histories, board.tick - time).cells.entries())}
    times.get(time)?.forEach(newCell => {
      newBoard.cells.set(JSON.stringify({ x: newCell.p.x, y: newCell.p.y }), newCell.c)
    })
    return {board: newBoard, outputs}
  }

  // add old cells that are not deleted and not updated
  cells.forEach((cell, pointStr) => {
    const point = JSON.parse(pointStr)
    if (deleted.find(p => p.x === point.x && p.y === point.y) === undefined) {
      if (newCells.get(pointStr) === undefined) {
        newCells.set(pointStr, cell)
      }
    }
  })
  return {board: { tick: board.tick + 1, cells: newCells }, outputs}
}

export const applyAB = (board: Board, a: number, b: number): Board => {
  const newCells = new Map<string, Cell>([...board.cells.entries()])
  board.cells.forEach((cell, pointStr) => {
    if (cell.type === 'A') {
      newCells.set(pointStr, { type: 'Int', value: a })
    }
    if (cell.type === 'B') {
      newCells.set(pointStr, { type: 'Int', value: b })
    }
  })
  return { tick: board.tick, cells: newCells }
}

export const findHistory = (histories: Board[], tick: number): Board => {
  for (const history of [...histories].reverse()) {
    if (history.tick === tick) {
      return history
    }
  }
  throw new Error("History not found");
}

