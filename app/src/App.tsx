import { useEffect, useRef, useState } from "react";
import "./app.css";

function App() {
  const canvasRef = useRef(null);

  const [x, setX] = useState(0);
  const [y, setY] = useState(0);

  const display = (e: MouseEvent) => {
    setX(e.clientX);
    setY(e.clientY);
  };

  function getRandomColor() {
    const letters = "0123456789ABCDEF";
    let color = "#";
    for (let i = 0; i < 6; i++) {
      color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
  }

  const [running, setRunning] = useState<boolean>(false);
  const [id, setID] = useState<number>(0);
  const handler = () => {
    if (running) {
      clearInterval(id);
    } else {
      setID(setInterval(wtf, 1000));
    }
    setRunning(!running);
  };

  //

  const pixelDrawing = (x, y, r, g, b, a) => {
    const ctx: CanvasRenderingContext2D = canvasRef.current.getContext("2d");
    let canvasData: ImageData = ctx.getImageData(
      0,
      0,
      ctx.canvas.width,
      ctx.canvas.height,
    );

    let index: number = (x + y * ctx.canvas.width) * 4;

    canvasData.data[index + 0] = r;
    canvasData.data[index + 1] = g;
    canvasData.data[index + 2] = b;
    canvasData.data[index + 3] = a;
  };

  const updateCanvas = () => {
    const ctx: CanvasRenderingContext2D = canvasRef.current.getContext("2d");
    let canvasData: ImageData = ctx.getImageData(
      0,
      0,
      ctx.canvas.width,
      ctx.canvas.height,
    );
    ctx.putImageData(canvasData, 0, 0);
  };

  const wtf = () => {
    const ctx: CanvasRenderingContext2D = canvasRef.current.getContext("2d");
    console.time("Creation");
    const array = Array.from({ length: 800 }, () => Array(800).fill(0));
    console.timeEnd("Creation");

    console.time("Array");
    for (let i = 0; i < array.length; i++) {
      for (let j = 0; j < array[i].length; j++) {
        // ctx.fillStyle = "000000";
        // ctx.fillStyle = getRandomColor();
        // ctx.fillRect(i, j, 1, 1);
        console.log("Hello");
        pixelDrawing(i, j, 0, 0, 0, 0);
      }
    }
    updateCanvas();
    console.timeEnd("Array");
  };

  useEffect(() => {
    window.addEventListener("mousemove", display);
    const ctx: CanvasRenderingContext2D = canvasRef.current.getContext("2d");
    console.time();
    ctx.fillRect(500, 500, 50, 50);
    console.timeEnd();

    return () => {
      window.removeEventListener("mousemove", display);
    };
  }, []);

  return (
    <>
      <p> This is X: {x} </p>
      <p> This is Y: {y} </p>
      <p>
        {" "}
        Window Resolution: {window.innerWidth} by {window.innerHeight}{" "}
      </p>
      <button onClick={handler}> Click </button>

      <canvas
        width={window.innerWidth}
        height={window.innerHeight}
        ref={canvasRef}
      />
    </>
  );
}

export default App;
