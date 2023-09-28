// JavaScript code
export function getCurrentTime() {
    const currentTime = new Date();
    return currentTime.toISOString(); // Example format: "2023-09-28T12:34:56.789Z"
}


function startTimer() {
    setInterval(function () {
        // Call the Wasm function to trigger a change
        wasmModule.triggerChange();
    }, 1000); // Interval in milliseconds (e.g., 1000ms = 1 second)
}

// Start the timer when your application loads
startTimer();
