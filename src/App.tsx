import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [message, setMessage] = useState("");
  const [operationEnabled, setOperationEnabled] = useState(true);

  const disabledButtonStyle = {
    backgroundColor: "#A9A9A9",
    cursor: "not-allowed",
  };

  async function button1_pushed() {
    await invoke("button1_pushed", { message: message });
  }

  async function button2_pushed() {
    await invoke("button2_pushed", { message: message });
  }

  useEffect(() => {
    const unlistenPromise = listen<boolean>(
      "operation-enabled-updated",
      (event) => {
        setOperationEnabled(event.payload);
      }
    );
    return () => {
      void unlistenPromise.then((unlistenFn) => {
        unlistenFn();
      });
    };
  }, []);

  return (
    <div className="container">
      <form className="row">
        <input
          id="message-input"
          onChange={(e) => setMessage(e.currentTarget.value)}
          placeholder="Enter a message..."
        />
        <button
          style={!operationEnabled ? disabledButtonStyle : {}}
          disabled={!operationEnabled}
          onClick={() => button1_pushed()}
        >
          Publish1
        </button>
        <button
          style={!operationEnabled ? disabledButtonStyle : {}}
          disabled={!operationEnabled}
          onClick={() => button2_pushed()}
        >
          Publish2
        </button>
      </form>
    </div>
  );
}

export default App;
