import socket
import json
import time

def connect(port):
   sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
   sock.connect(('127.0.0.1', port))
   return sock

request = {
   "queueUpRequest": {
      "nickname": "ziv caspi"
   }
}


b = bytes(json.dumps(request), 'utf-8') + b'\n'
print(type(b),b)
s = connect(7878)
time.sleep(3)
# s.send(b)
response = s.recv(1024)
print('response:', response)