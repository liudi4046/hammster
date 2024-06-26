import React, { useState, useEffect } from "react";
import * as hm from "./lib/wasm/hammster2.js";

function FileUpload() {
  const [file, setFile] = useState(null);
  const [wasmReady, setWasmReady] = useState(false);
  const [hammingDistance, setHammingDistance] = useState(0);

  useEffect(() => {
    async function loadWasm() {
      try {
        await hm.default();
        setWasmReady(true);
      } catch (error) {
        console.error("Error loading wasm module:", error);
        alert("Failed to load the WebAssembly module.");
      }
    }

    loadWasm();
  }, []);

  const onFileChange = (event) => {
    setFile(event.target.files[0]);
  };

  const onVerifyClick = async () => {
    if (!file) {
      alert("Please select a file to verify.");
      return;
    }

    if (!wasmReady) {
      alert("WASM module is not ready yet.");
      return;
    }
    console.log("before public input:", hammingDistance);

    const reader = new FileReader();
    reader.onload = async (event) => {
      const fileContent = new Uint8Array(event.target.result);
      console.log("File content:", fileContent);

      let public_input = new Uint32Array([hammingDistance]);
      console.log("now public input:", public_input);

      try {
        hm.verify(public_input, fileContent);
        alert(`Verification success`);
      } catch (error) {
        console.error("Verification failed:", error);
        alert("Verification failed.");
      }
    };
    reader.readAsArrayBuffer(file);
  };
  const onHammingDistanceChange = (e) => {
    setHammingDistance(e.target.value);
  };

  const containerStyle = {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: "100vh",
    backgroundColor: "#f5f5f5",
    padding: "50px",
    boxSizing: "border-box",
  };

  const inputStyle = {
    margin: "10px 0",
    padding: "10px",
    fontSize: "16px",
  };

  const buttonStyle = {
    padding: "10px 20px",
    fontSize: "16px",
    backgroundColor: "#4CAF50",
    color: "white",
    border: "none",
    borderRadius: "5px",
    cursor: "pointer",
  };

  return (
    <div style={containerStyle}>
      <input type="file" onChange={onFileChange} style={inputStyle} />
      <input
        type="number"
        value={hammingDistance}
        onChange={onHammingDistanceChange}
        style={inputStyle}
        placeholder="Enter Hamming Distance"
      />

      <button onClick={onVerifyClick} style={buttonStyle}>
        Verify
      </button>
    </div>
  );
}

export default FileUpload;
