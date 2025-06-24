import socket
import hashlib

# ==== Protocol Constants (must match server) ====
CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'

# ==== Server Configuration ====
SERVER_IP = "127.0.0.1"
SERVER_PORT = 9999
CLIENT_KEY = "WHAT"  # Shared key (must match server)

def main():
    try:
        # Create socket and connect
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((SERVER_IP, SERVER_PORT))
        print("[+] Connected to server")

        # Step 1: Check server status
        sock.sendall(CHECK_SERVER_MSG)
        response = sock.recv(1024)
        if response != SERVER_IS_UP_MSG:
            print("[-] Unexpected response to CHECK_SERVER_MSG")
            sock.close()
            return
        print("[+] Server is up")

        # Step 2: Send authentication key hash
        key_hash = hashlib.sha256(CLIENT_KEY.encode()).digest()
        sock.sendall(CLIENT_INIT_CONN_KEY_MSG + key_hash)
        print("[+] Sent client key hash")

        # Step 3: Receive handshake result
        response = sock.recv(len(KEY_EXCHANGE_SUCCEEDED_MSG))
        if response == KEY_EXCHANGE_FAILED_MSG:
            print("[-] Key exchange failed")
            sock.close()
            return
        elif response != KEY_EXCHANGE_SUCCEEDED_MSG:
            print("[-] Unexpected handshake response")
            sock.close()
            return
        print("[+] Authenticated with server")

        # === Command loop ===
        print("[*] Ready to receive/send messages. Type 'exit' to quit.")
        while True:
            # Receive server message
            sock.settimeout(0.2)
            try:
                data = sock.recv(4096)
                if data:
                    print("Server:", data.decode())
            except socket.timeout:
                pass  # no data received, continue

            # Input message to send
            msg = input("You > ").strip()
            if msg.lower() == "exit":
                break
            if msg:
                sock.sendall(msg.encode())

        sock.close()
        print("[*] Connection closed.")

    except Exception as e:
        print(f"[-] Error: {e}")


if __name__ == "__main__":
    main()
