import { useEffect, useState } from 'react'
import './App.css'
import { Board, applyAB, parseBoard, printCell, tickBoard } from './lib'

function App() {
  const [board, setBoard] = useState('')
  const [history, setHistory] = useState<Board[]>([])
  const [output, setOutput] = useState('')
  const [scale, setScale] = useState(0)
  const [minX, setMinX] = useState(0)
  const [maxX, setMaxX] = useState(0)
  const [minY, setMinY] = useState(0)
  const [maxY, setMaxY] = useState(0)
  const [A, setA] = useState(0)
  const [B, setB] = useState(0)


  useEffect(() => {
    const parsed = parseBoard(board)
    console.log(parsed)
    if (parsed.cells.size === 0) {
      return
    }
    var current = applyAB(parsed, A, B)
    var hist = [parsed]
    // while (true) {
    for (let i = 0; i < 10000; i++) {
      try {
        const { board: next, outputs } = tickBoard(current, hist)
        hist = [...hist, next]
        current = next
        if (outputs.length > 0) {
          setOutput(outputs.map(o => o.type == 'Int' ? o.value.toString() : '').join('\n'))
          break
        }
      } catch (e: any) {
        window.alert(e.message)
        break
      }
    }
    var minx = 0
    var maxx = 0
    var miny = 0
    var maxy = 0
    for (const h of hist) {
      const { cells } = h
      const xs = Array.from(cells.keys()).map(p => JSON.parse(p).x)
      const ys = Array.from(cells.keys()).map(p => JSON.parse(p).y)
      minx = Math.min(minx, ...xs)
      maxx = Math.max(maxx, ...xs)
      miny = Math.min(miny, ...ys)
      maxy = Math.max(maxy, ...ys)
    }
    setMinX(minx)
    setMaxX(maxx)
    setMinY(miny)
    setMaxY(maxy)
    setHistory(hist)
  }, [board, A, B])

  return (
    <>
      <form>
        <textarea id='solution' defaultValue={board}/>
        <input id='A' type='number' defaultValue={A} onChange={(e) => {
          e.preventDefault()
        }} />
        <input id='B' type='number' defaultValue={B} onChange={(e) => {
          e.preventDefault()
        }} />
        <button onClick={e => {
          e.preventDefault()
          const text = document.getElementById('solution') as HTMLTextAreaElement
          setBoard(text.value)
          const a = document.getElementById('A') as HTMLInputElement
          setA(parseInt(a.value))
          const b = document.getElementById('B') as HTMLInputElement
          setB(parseInt(b.value))
        }}>Submit</button>
      </form>
      <input id='scale' type="range" min="0" max={history.length-1} step="1" defaultValue={scale} onChange={(e) => {
          setScale(parseInt(e.target.value))
        }} />
      {history.length > 0 ? (() => {
        const b = history[scale]
        const { cells } = b
        const range = (start: number, end: number) => [...Array(end + 1).keys()].slice(start);

        return <div>
          <code>tick: {b.tick}</code>
          <table>
          {
            range(minY, maxY).map(y => (
              <tr>
                {range(minX, maxX).map(x => {
                  const cell = cells.get(JSON.stringify({ x, y })) || { type: 'Empty' }
                  return <td><code>{printCell(cell)}</code></td>
                })}
              </tr>
            ))
          }
        </table>
          </div>
      })() : <></>}
      <div>{output}</div>
    </>
  )
}

export default App
