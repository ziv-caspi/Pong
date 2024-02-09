import socket
import json
import time
import pygame

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
      return parsed
   return None

def try_potential_match_update(response: dict):
   match_update = response.get('serverPushUpdate').get('potentialMatchUpdate')
   if match_update:
      return (match_update['matchId'], match_update['opoonentsIds'])
   else:
      return (None, None)

def enter_match(s):
   print('enter nickname:')
   name = input()
   id = queue_up(name)
   print('user id:', id)

   while True:
      update = no_updates()
      if not update:
         continue
      
      (match_id, players) = try_potential_match_update(update)
      if match_id:
         user_in = ''
         print('Found a match for you!')
         while user_in != 'GO':
            print('Type GO to join lobby')
            user_in = input()
            join_lobby(match_id)

def start_game():
   # pygame setup
   pygame.init()
   screen = pygame.display.set_mode((1280, 720))
   clock = pygame.time.Clock()
   running = True
   dt = 0

   player_pos = pygame.Vector2(screen.get_width() / 15, screen.get_height() / 2)

   while running:
      for event in pygame.event.get():
         if event.type == pygame.QUIT:
               running = False

      screen.fill('black')

      pygame.draw.rect(surface=screen, color=(136, 242, 139), rect=(player_pos, (10, 150)))
      #pygame.draw.circle(screen, "red", player_pos, 40)

      pos_delta = 300 * dt
      moved = False
      
      keys = pygame.key.get_pressed()
      if keys[pygame.K_UP]:
         pos_delta *= -1
         moved=True
      if keys[pygame.K_DOWN]:
         pos_delta *= 1
         moved=True

      if moved:
         player_pos.y += pos_delta

      pygame.display.flip()
      dt = clock.tick(60) / 1000

   pygame.quit()


def main():
   start_game()
   return
   s = connect(7878)
   enter_match(s)


if __name__ == '__main__':
   main()
      