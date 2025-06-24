import commands_handler
import socket
import sys
import threading
import time
import hashlib
import json
# =======
#defining messaging protocol constants:
CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'


# like [b'CHECK_SERVER_MSG', b'SERVER_IS_UP_MSG']
# to sortage connection level
client_states = {}  


# handle for socket  => like [<socket.socket fd=7>, <socket.socket fd=8>]
connections = []

# sortage address => like  [('192.168.1.10', 50322), ('192.168.1.11', 50325)]
addresses = []

# to shutdown the threads client
Shutdown_Flag = threading.Event()

# if the server press listen ... listen is start 
listen_event = threading.Event()

# sortage the client address =>  like current_client = ('192.168.1.10', 50322)
global current_client
current_client = None

# to lock the liste addresses (if thread "A" write it .. another thread "B" can not write it )
conn_lock = threading.Lock()

#Authorization key
key = "password"

# 4 Byte hash like => server_key_hash = b'\xa5\x89\xdf...\x91
server_key_hash = hashlib.sha256(key.encode()).digest()  



def sock():
    try:
        print(commands_handler.entro())
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        # release port now when the thread is stoped
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
            break
    s.close()



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
                print(commands_handler.help_msg_func())
                
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
        # command = ls -la /home
        # parts = [ "ls" , "-la" , "/home" ]
        parts = shlex.split(cmd)

        command = parts[0] # ls
        context = parts[1:] if len(parts) > 1 else [] # context = ["-la" , "/home"]

        
        payload = {
            "command": command,
            "args": context,
            "flags": []
        }
        # payload = {"command" : "ls" , "args : ["/home"]" , flags : ["-la"] }

        payload["flags"] = [flag for flag in context if flag.startswith('-')]
        payload["args"] = [arg for arg in context if not arg.startswith('-')]
        
      
        
        
        # convert the length of payload_bytes to a 4-byte binary representation
        payload_str = json.dumps(payload)

        # and then send the length prefix followed by the JSON payload
        payload_bytes = payload_str.encode()

        # this is to inform the client of the exact size that he needs to read.
        payload_length = len(payload_bytes).to_bytes(4, byteorder='big')
        
        conn.send(payload_length + payload_bytes)

        # Receive response with length prefix:
        response_length_bytes = conn.recv(4)
        if not response_length_bytes:
            raise ConnectionError("No response received from client")
        
        #convert the length prefix to an integer:
        response_length = int.from_bytes(response_length_bytes, byteorder='big')
        
        # Receive the full response with help of recv_all function:
        response_data = recv_all(conn, response_length)
        client_response = response_data.decode()
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
    except Exception as e:
        print(f"Error sending command: {e}")
        raise



def recv_all(conn, expected_length):
    data = b''
    while len(data) < expected_length:
        packet = conn.recv(expected_length - len(data))
        if not packet:
            break
        data += packet
    return data


### Cleans up all active client connections and closes the server socket:
def cleanup(sock):
    with conn_lock:
        for conn in connections:
            conn.close()
    sock.close()

def main():
    s = sock()
    conn_thread = threading.Thread(target=Connection_Handling, args=(s,),daemon=True)
    conn_thread.start()
    try:
        turtle()
    finally:
        Shutdown_Flag.set()
        cleanup(s)

main()



