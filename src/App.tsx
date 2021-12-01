import { useEffect } from "react";
import init from "wasm";
import "./App.css";

function App() {
  useEffect(() => {
    init();
  }, []);

  return <canvas id="canvas" width={1024} height={720}></canvas>;
}

export default App;
