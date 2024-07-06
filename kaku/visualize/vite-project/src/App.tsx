import { useEffect, useState } from 'react'
import './App.css'
import { Board, applyAB, calcScore, parseBoard, printCell, tickBoard } from './lib'

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
  const [maxTick, setMaxTick] = useState(0)
  const [score, setScore] = useState(0)

  useEffect(() => {
    const parsed = parseBoard(board)
    console.log(parsed)
    if (parsed.cells.size === 0) {
      return
    }
    var current = applyAB(parsed, A, B)
    var hist = [current]
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
    var maxt = 0
    for (const h of hist) {
      const { cells, tick } = h
      const xs = Array.from(cells.keys()).map(p => JSON.parse(p).x)
      const ys = Array.from(cells.keys()).map(p => JSON.parse(p).y)
      minx = Math.max(Math.min(minx, ...xs), -200)
      maxx = Math.min(Math.max(maxx, ...xs), 200)
      miny = Math.max(Math.min(miny, ...ys), -200)
      maxy = Math.min(Math.max(maxy, ...ys), 200)
      maxt = Math.max(maxt, tick)
    }
    setMinX(minx)
    setMaxX(maxx)
    setMinY(miny)
    setMaxY(maxy)
    setHistory(hist)
    setMaxTick(maxt)
    setScore(calcScore(hist.slice(0, hist.length-1)))
  }, [board, A, B])

  return (
    <>
      <form>
        <div>
          <textarea id='solution' defaultValue={board}/>
        </div>
        <div>
          A: <input id='A' type='number' defaultValue={A} onChange={(e) => {
            e.preventDefault()
          }} />
        </div>
        <div>
          B: <input id='B' type='number' defaultValue={B} onChange={(e) => {
            e.preventDefault()
          }} />
        </div>
        <button onClick={e => {
          e.preventDefault()
          const text = document.getElementById('solution') as HTMLTextAreaElement
          setBoard(text.value)
          const a = document.getElementById('A') as HTMLInputElement
          setA(parseInt(a.value))
          const b = document.getElementById('B') as HTMLInputElement
          setB(parseInt(b.value))
          setScale(0)
        }}>Execute</button>
      </form>
      <div onKeyDown={e => {
        if (e.key === 'ArrowLeft') {
          setScale(Math.max(0, scale-1))
        } else if (e.key === 'ArrowRight') {
          setScale(Math.min(history.length-1, scale+1))
        }
      }} className='range-bar'>
        <button onClick={e => {
          e.preventDefault()
          setScale(Math.max(0, scale-1))
        }}>Prev</button>
        <input id='scale' type="range" min="0" max={history.length-1} step="1" value={scale} onChange={(e) => {
            setScale(parseInt(e.target.value))
          }} />
        <button onClick={e => {
          e.preventDefault()
          setScale(Math.min(history.length-1, scale+1))
        }}>Next</button>
      </div>
      <div>{scale} / {history.length-1}</div>
      <div>maxTick: {maxTick}</div>
      <div>volume: {score}</div>
      <div>output: {output}</div>
      {history.length > 0 ? (() => {
        const b = history[scale]
        const { cells } = b
        const range = (start: number, stop: number) => Array.from({ length: (stop - start) + 1}, (_, i) => start + i);

        return <div>
          <div>tick: {b.tick}</div>
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
          <div>tick: {b.tick}</div>
          </div>
      })() : <></>}
      <div onKeyDown={e => {
        if (e.key === 'ArrowLeft') {
          setScale(Math.max(0, scale-1))
        } else if (e.key === 'ArrowRight') {
          setScale(Math.min(history.length-1, scale+1))
        }
      }} className='range-bar'>
        <button onClick={e => {
          e.preventDefault()
          setScale(Math.max(0, scale-1))
        }}>Prev</button>
        <input id='scale' type="range" min="0" max={history.length-1} step="1" value={scale} onChange={(e) => {
            setScale(parseInt(e.target.value))
          }} />
        <button onClick={e => {
          e.preventDefault()
          setScale(Math.min(history.length-1, scale+1))
        }}>Next</button>
      </div>
      <div>{scale} / {history.length-1}</div>
    </>
  )
}

export default App
