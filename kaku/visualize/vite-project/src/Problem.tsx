import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Board, applyAB, calcScore, parseBoard, tickBoard } from "./lib";
import './Problem.css'

type ProblemSpec = {
  question: string,
  instances: {
    input: {A: number, B: number},
    output: number
  }[]
}

export const problems: ProblemSpec[] = [
  {
    question: `Output the A to the power of B.\n-100 <= A <= 100, 0 <= B <= 10`,
    instances: [
      { input: { A: 1, B: 2 }, output: 1 },
      { input: { A: 2, B: 4 }, output: 16 },
      { input: { A: 3, B: 10 }, output: 59049 },
      { input: { A: -5, B: 6 }, output: 15625 },
      { input: { A: -5, B: 5 }, output: -3125 },
      { input: { A: 5, B: 0 }, output: 1 },
      { input: { A: -5, B: 0 }, output: 1 },
      { input: { A: 10, B: 1 }, output: 10 }
    ]
  }
]

function Problem() {
  const { id } = useParams();
  const problemSpec: ProblemSpec =  problems[parseInt(id!)-1]
  const [message, setMessage] = useState<string>('')
  const [score, setScore] = useState<number>(0)
  const [solution, setSolution] = useState<string>('')
  const [isPopUpVisible, setPopUpVisible] = useState(false);

  const togglePopUp = () => {
    setPopUpVisible(!isPopUpVisible);
  };

  const check = (b: Board, instance: {
    input: {A: number, B: number},
    output: number
  }): {ac: boolean, score: number} => {
      var current = applyAB(b, instance.input.A, instance.input.B)
      var hist = [current]
      for (let i = 0; i < 10000; i++) {
        try {
          const { board: next, outputs } = tickBoard(current, hist)
          if (outputs.length == 1) {
            if (outputs[0].type == 'Int' && outputs[0].value == instance.output) {
              return {ac: true, score: calcScore(hist.slice(0, hist.length-1))}
            } else {
              return {ac: false, score: 0};
            }
          } else if (outputs.length > 0) {
            return {ac: false, score: 0};
          }
          hist = [...hist, next]
          current = next
        } catch (e: any) {
          return {ac: false, score: 0};
        }
      }
      return {ac: false, score: 0};
  }

  useEffect(() => {
    const parsed = parseBoard(solution)
    if (parsed.cells.size === 0) {
      setMessage('WA')
      return
    }
    var scoreSum = 0
    for (const instance of problemSpec.instances) {
      const result = check(parsed, instance)
      if (result.ac) {
        scoreSum += result.score
      } else {
        setMessage('WA')
        setScore(0)
        togglePopUp()
        return
      }
    }
    setScore(scoreSum)
    setMessage('AC')
    togglePopUp()
  }, [solution])

  return (
    <>
      <h2>Problem {id}</h2>
      <p>{problemSpec.question.split(`\n`).map(s => <>{s}<br/></>)}</p>


      {isPopUpVisible && (
        <div className='popup'>
          <p>{message}</p>
          <p>Score: {score}</p>
          <button onClick={togglePopUp}>Close</button>
        </div>
      )}

      <form>
        <div>
          <textarea id='board'></textarea>
        </div>
        <button onClick={e => {
          e.preventDefault()
          const board = document.getElementById('board') as HTMLTextAreaElement
          setSolution(board.value)
        }}>Submit</button>
      </form>
    </>
  )
}

export default Problem
