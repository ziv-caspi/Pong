import socket
import json
import time

def connect(port):
   sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
   sock.connect(('127.0.0.1', port))
   return sock

def prepare_request(req):
   return bytes(json.dumps(req), 'utf-8') + b'\n'

def parse_response(resp):
   return json.loads(resp)

queue_up_request = {
   "queueUpRequest": {
      "nickname": "ziv caspi"
   }
}

no_updates_request = 'noUpdates'


s = connect(7878)
time.sleep(3)
s.send(prepare_request(queue_up_request))
response = s.recv(1024)
print('response:', response)
while True:
   s.send(prepare_request(no_updates_request))
   response = s.recv(1024)
   parsed = parse_response(response)
   if parsed['serverPushUpdate']:
      print('response:', response)