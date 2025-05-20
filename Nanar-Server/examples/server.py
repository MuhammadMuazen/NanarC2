import socket

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
                    conn.send(data)
            finally:
                conn.close()
                
    except Exception as e:
        print(f"[-] Error: {e}")
    finally:
        if server_sock:
            server_sock.close()
