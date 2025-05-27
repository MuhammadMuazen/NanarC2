import socket

CHECK_SERVER_MSG = b'CHECK_SERVER_MSG'
SERVER_IS_UP_MSG = b'SERVER_IS_UP_MSG'
SERVER_IS_DOWN_MSG = b'SERVER_IS_DOWN_MSG'
CLIENT_INIT_CONN_KEY_MSG = b'CLIENT_INIT_CONN_KEY_MSG'
KEY_EXCHANGE_SUCCEEDED_MSG = b'KEY_EXCHANGE_SUCCEEDED_MSG'
KEY_EXCHANGE_FAILED_MSG = b'KEY_EXCHANGE_FAILED_MSG'
#hard beat message response 
#hard beat check connection 

DONE_INIT_PROCESS_LIST = []

if __name__ == "__main__":
    server_sock = None
    try:
        # Create server socket
        server_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    
        print('[+] Listening on 127.0.0.1:9999')
        server_sock.bind(("127.0.0.1", 9999))
        server_sock.listen(5)
        
        while True:
            conn, addr = server_sock.accept()
            print(f'Connection from {addr}')
            
            try:
                while True:
                    data = conn.recv(1024)
                    if not data:
                        break
                    print(f'data: {data}')
                    
                    if data == CHECK_SERVER_MSG:
                        DONE_INIT_PROCESS_LIST.append(CHECK_SERVER_MSG)
                        print("[+] Checking if the server is up")
                        conn.send(SERVER_IS_UP_MSG)
                        DONE_INIT_PROCESS_LIST.append(SERVER_IS_UP_MSG)
                    elif data == CLIENT_INIT_CONN_KEY_MSG and DONE_INIT_PROCESS_LIST[len(DONE_INIT_PROCESS_LIST) - 1] == SERVER_IS_UP_MSG:
                        DONE_INIT_PROCESS_LIST.append(CLIENT_INIT_CONN_KEY_MSG)
                        print(f'[+] Got the inilization key from the client')
                        conn.send(KEY_EXCHANGE_SUCCEEDED_MSG)
                        print("[+] Sent the KEY_EXCHANGE_SUCCEEDED_MSG")
                        DONE_INIT_PROCESS_LIST.append(KEY_EXCHANGE_SUCCEEDED_MSG)
                    elif data != CLIENT_INIT_CONN_KEY_MSG and DONE_INIT_PROCESS_LIST[len(DONE_INIT_PROCESS_LIST) - 1] == SERVER_IS_UP_MSG:
                        DONE_INIT_PROCESS_LIST.append(CLIENT_INIT_CONN_KEY_MSG)
                        print(f'[+] Got the inilization key from the client and it is wrong: {data}')
                        conn.send(KEY_EXCHANGE_FAILED_MSG)
                        print("[+] Send the KEY_EXCHANGE_FAILED_MSG")
                        DONE_INIT_PROCESS_LIST.append(KEY_EXCHANGE_FAILED_MSG)
            finally:
                conn.close()      
    except Exception as e:
        print(f"[-] Error: {e}")
    finally:
        if server_sock:
            server_sock.close()
