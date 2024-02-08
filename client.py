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

def queue_up(nickname):
   queue_up_request = {
      "queueUpRequest": {
         "nickname": nickname
      }
   }

   req = prepare_request(queue_up_request)
   s.send(req)
   response = s.recv(1024)
   parsed = parse_response(response)
   print(parsed)
   return parsed['queueUpResponse']['Ok']['id']

def join_lobby(match_id):
   join_lobby_request = {
      "joinLobbyRequest": {
         "matchId": match_id
      }
   }

   req = prepare_request(join_lobby_request)
   s.send(req)
   response = s.recv(1024)
   parsed: dict = parse_response(response)
   print(parsed)
   response: dict = parsed.get('joinLobbyResponse')
   if not response.get('Ok'):
      print('could not join lobby:', response)
      return False
   
   print('Joined Lobby, Waiting For Other players')
   return True


def no_updates():
   no_updates_request = 'noUpdates'
   req = prepare_request(no_updates_request)
   s.send(req)
   response = s.recv(1024)
   parsed = parse_response(response)
   if parsed['serverPushUpdate']:
      print(parsed)
      return parsed
   return None

def try_potential_match_update(response: dict):
   match_update = response.get('serverPushUpdate').get('potentialMatchUpdate')
   if match_update:
      return (match_update['matchId'], match_update['opoonentsIds'])
   else:
      return (None, None)


s = connect(7878)
print('enter nickname:')
name = input()
id = queue_up(name)
print('user id:', id)

while True:
   update = no_updates()
   if not update:
      continue
   
   (match_id, players) = try_potential_match_update(update)
   if id:
      user_in = ''
      print('Found a match for you!')
      while user_in != 'GO':
         print('Type GO to join lobby')
         user_in = input()
         join_lobby(match_id)
      


# time.sleep(3)
# s.send(prepare_request(queue_up_request))
# response = s.recv(1024)
# print('response:', response)
# while True:
#    s.send(prepare_request(no_updates_request))
#    response = s.recv(1024)
#    parsed = parse_response(response)
#    if parsed['serverPushUpdate']:
#       print('response:', response)