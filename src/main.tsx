import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import './App.css';

const meta = document.createElement('meta');
meta.name = "viewport";
meta.content = "width=device-width, initial-scale=1.0, viewport-fit=cover";
document.getElementsByTagName('head')[0].appendChild(meta);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
