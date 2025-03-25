import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [result, setResult] = useState("");
  const [numbers, setNumbers] = useState("");

  async function calculateFibonacci() {
    try {
      const numberList = numbers.split(/[^0-9]/).map(Number);

      const fibResults = await invoke("fibonacci", { numbers: numberList });

      const resultText = numberList
        .map((n, index) => `Fibonacci of ${n} is ${fibResults[index]}`)
        .join("\n");
      setResult(resultText);
    } catch (error) {
      setResult(
        "Error calculating the fibonacci. Please enter valid number(s) separated by commas."
      );
    }
  }

  return (
    <main className="container">
      <h1>Welcome to Fibonacci Calculator</h1>

      <p>Click on the Vite, Tauri and React logos to learn more.</p>
      <div className="row logos">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src="/react.svg" className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Enter the number(s) separated by a comma(,) and click Calculate.</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          calculateFibonacci();
        }}
      >
        <input
          id="fibonacci-input"
          onChange={(e) => setNumbers(e.currentTarget.value)}
          placeholder="Enter numbers"
        />
        <button type="submit">Calculate</button>
      </form>
      <pre className="result">{result}</pre>
    </main>
  );
}

export default App;