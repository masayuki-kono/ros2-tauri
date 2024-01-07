import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [message, setMessage] = useState("");

  async function publish() {
    await invoke("button_pushed", { message: message });
  }

  return (
    <div className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          publish();
        }}
      >
        <input
          id="message-input"
          onChange={(e) => setMessage(e.currentTarget.value)}
          placeholder="Enter a message..."
        />
        <button type="submit">Publish</button>
      </form>
    </div>
  );
}

export default App;
