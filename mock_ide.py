import socket
import json
import time

def send_command(s, cmd):
    s.sendall((json.dumps(cmd) + "\n").encode())

def listen_events(s):
    s.setblocking(0)
    try:
        data = s.recv(4096).decode()
        if data:
            print(f"📥 Received: {data.strip()}")
            return [json.loads(line) for line in data.splitlines() if line.strip()]
    except BlockingIOError:
        pass
    return []

def main():
    print("🔌 Connecting to debug server on localhost:9000...")
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        s.connect(("localhost", 9000))
    except ConnectionRefusedError:
        print("❌ Could not connect. Is the runner in debug mode?")
        return

    # 1. Set breakpoint on line 3
    print("📍 Setting breakpoint on line 3...")
    send_command(s, {"SetBreakpoints": {"file": "debug_test.dryad", "lines": [3]}})
    
    # 2. Continue
    print("▶️ Continuing execution...")
    send_command(s, "Continue")
    
    # 3. Wait for breakpoint hit
    while True:
        events = listen_events(s)
        for ev in events:
            if "BreakpointHit" in ev:
                print(f"✅ Hit breakpoint at line {ev['BreakpointHit']['line']}")
                
                # 4. Get variables
                print("🔍 Requesting variables...")
                send_command(s, "GetVariables")
            
            if "Variables" in ev:
                print(f"📊 Current Variables: {ev['Variables']}")
                
                # 5. Step
                print("👞 Stepping...")
                send_command(s, "Step")
            
            if "StepComplete" in ev:
                print(f"✅ Step complete at line {ev['StepComplete']['line']}")
                print("▶️ Continuing to end...")
                send_command(s, "Continue")
                time.sleep(1)
                return

        time.sleep(0.1)

if __name__ == "__main__":
    main()
