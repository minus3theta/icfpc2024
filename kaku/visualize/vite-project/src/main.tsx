import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import Problem, { problems } from './Problem.tsx'
import './index.css'
import { BrowserRouter as Router, Route, Link, Routes } from 'react-router-dom';

const problemNum = problems.length

const Problems = () => {
  return (
    <div>
      {Array.from({length: problemNum}, (_, i) => i + 1).map(i => (
        <div key={i}>
          <Link to={`/problem/${i}`}>Problem {i}</Link>
        </div>
      ))}
    </div>
  )

}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Router>
      <nav>
        <ul>
          <li>
            <Link to="/">Simulator</Link>
          </li>
          <li>
            <Link to="/problems">Problems</Link>
          </li>
        </ul>
      </nav>

      <div>
        <a href="https://github.com/icfpcontest2024/icfpc2024/blob/main/static/3d/3d.md">3D Program Spec</a>
      </div>

      <div id='contents'>
        <div>
          <Routes>
            <Route path="/" element={<App/>} />
            <Route path="/problem/:id" element={<Problem />} />
            <Route path="/problems" element={<Problems />} />
          </Routes>
        </div>
      </div>
    </Router>
  </React.StrictMode>,
)
