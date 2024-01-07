import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [message, setMessage] = useState("");

  async function button1_pushed() {
    await invoke("button1_pushed", { message: message });
  }

  async function button2_pushed() {
    await invoke("button2_pushed", { message: message });
  }

  return (
    <div className="container">
      <form className="row">
        <input
          id="message-input"
          onChange={(e) => setMessage(e.currentTarget.value)}
          placeholder="Enter a message..."
        />
        <button onClick={() => button1_pushed()}>Publish1</button>
        <button onClick={() => button2_pushed()}>Publish2</button>
      </form>
    </div>
  );
}

export default App;
