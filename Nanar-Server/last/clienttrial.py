import socket
import hashlib

# Messaging protocol constants (must match the server's)
CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'

# Client configuration
SERVER_IP = "127.0.0.1"  # Server's IP address
SERVER_PORT = 9999        # Server's port
CLIENT_KEY = "password"   # Must match the server's 'key' variable

def main():
    try:
        # Create a socket and connect to the server
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client_socket.connect((SERVER_IP, SERVER_PORT))
        print("[+] Connected to server")

        # Step 1: Check if the server is up
        client_socket.send(CHECK_SERVER_MSG)
        response = client_socket.recv(1024)
        
        if response == SERVER_IS_UP_MSG:
            print("[+] Server is up")
        else:
            print("[-] Server is down or invalid response")
            client_socket.close()
            return

        # Step 2: Send the client's key hash
        client_key_hash = hashlib.sha256(CLIENT_KEY.encode()).digest()
        client_socket.send(CLIENT_INIT_CONN_KEY_MSG + client_key_hash)
        print("[+] Sent key hash")

        # Step 3: Check key exchange result
        key_response = client_socket.recv(1024)
        if key_response == KEY_EXCHANGE_SUCCEEDED_MSG:
            print("[+] Key exchange succeeded! Connected to server")
        elif key_response == KEY_EXCHANGE_FAILED_MSG:
            print("[-] Key exchange failed. Invalid key")
        else:
            print("[-] Unexpected response from server")

        # Close the connection after testing
        client_socket.close()

    except ConnectionRefusedError:
        print("[-] Connection refused. Is the server running and listening?")
    except Exception as e:
        print(f"[-] Error: {e}")
    finally:
        if 'client_socket' in locals():
            client_socket.close()

if __name__ == "__main__":
    main()