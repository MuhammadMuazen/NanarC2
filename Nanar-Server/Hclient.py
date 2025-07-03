import socket
import hashlib
import json
import time

# Constants (must match server)
CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'

HEARTBEAT_RETRY_CONNECTION_MSG = b'HEARTBEAT_RETRY_INIT_CONNECTION_MSG'
HEARTBEAT_SUCCESS_RESPONSE_MSG = b'HEARTBEAT_SUCCESS_RESPONSE_MSG'
HEARTBEAT_NO_ACTION_MSG = b'HEARTBEAT_NO_ACTION_MSG'
HEARTBEAT_NO_ACTION_RESPONSE_MSG = b'HEARTBEAT_NO_ACTION_RESPONSE_MSG'

# Server settings
SERVER_IP = '127.0.0.1'
SERVER_PORT = 9999
CLIENT_KEY = "WHAT"  # must match server

def send_handshake(sock):
    # Step 1: Check server status
    sock.sendall(CHECK_SERVER_MSG)
    resp = sock.recv(1024)
    if resp != SERVER_IS_UP_MSG:
        print("[-] Server not responding correctly")
        return False
    print("[+] Server is up")

    # Step 2: Send hashed key
    key_hash = hashlib.sha256(CLIENT_KEY.encode()).digest()
    sock.sendall(CLIENT_INIT_CONN_KEY_MSG + key_hash)
    resp = sock.recv(1024)

    if resp == KEY_EXCHANGE_SUCCEEDED_MSG:
        print("[+] Handshake successful")
        return True
    elif resp == KEY_EXCHANGE_FAILED_MSG:
        print("[-] Handshake failed: Invalid key")
    else:
        print("[-] Unexpected response during handshake")

    return False

def send_heartbeat(sock, mode="no_action"):
    if mode == "no_action":
        sock.sendall(HEARTBEAT_NO_ACTION_MSG)
        resp = sock.recv(1024)
        if resp == HEARTBEAT_NO_ACTION_RESPONSE_MSG:
            print("[+] Heartbeat OK (no action)")
        else:
            print("[-] Unexpected heartbeat response")
    elif mode == "retry":
        sock.sendall(HEARTBEAT_RETRY_CONNECTION_MSG)
        resp1 = sock.recv(1024)
        resp2 = sock.recv(1024)
        if resp1 == HEARTBEAT_SUCCESS_RESPONSE_MSG and resp2 == SERVER_IS_UP_MSG:
            print("[+] Reconnect triggered and acknowledged")
        else:
            print("[-] Unexpected reconnect response")

def send_command(sock, command_string):
    parts = command_string.strip().split()
    command = parts[0]
    args = parts[1:]

    payload = {
        "command": command,
        "args": [],
        "flags": []
    }
    payload["flags"] = [arg for arg in args if arg.startswith("-")]
    payload["args"] = [arg for arg in args if not arg.startswith("-")]

    payload_bytes = json.dumps(payload).encode()
    payload_length = len(payload_bytes).to_bytes(4, 'big')

    sock.sendall(payload_length + payload_bytes)

    response_length = int.from_bytes(sock.recv(4), 'big')
    response_data = b''
    while len(response_data) < response_length:
        chunk = sock.recv(response_length - len(response_data))
        if not chunk:
            break
        response_data += chunk

    print("[Server Response]:", response_data.decode())


def main():
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((SERVER_IP, SERVER_PORT))
        print("[+] Connected to server")

        if not send_handshake(sock):
            sock.close()
            return

        # Test heartbeat
        send_heartbeat(sock, mode="no_action")
        time.sleep(1)
        send_heartbeat(sock, mode="retry")

        # Optional: send commands if server supports it
        while True:
            cmd = input("Command > ").strip()
            if cmd.lower() in ["exit", "quit"]:
                break
            send_command(sock, cmd)

        sock.close()
    except Exception as e:
        print(f"[-] Error: {e}")

if __name__ == "__main__":
    main()
