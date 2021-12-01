import { useEffect } from "react";
import init from "wasm";
import "./App.css";

function App() {
  useEffect(() => {
    init();
  }, []);

  return (
    <main style={{ width: 1024, height: 768 }}>
      <canvas id="control-layer" width={1024} height={768} />
      <canvas id="paint-layer" width={1024} height={768} />
    </main>
  );
}

export default App;
