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

   pending = True
   while pending:
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
            pending = False

   print('finished joining match')
   return (id, match_id)

def move_player(pos_delta, match_id):
   #"{\"movePlayerRequest\":{\"matchId\":\"abc\",\"yDelta\":-5}}"
   move_player_request = {
      "movePlayerRequest": {
         "matchId": match_id,
         "yDelta": int(pos_delta)
      }
   }
   req = prepare_request(move_player_request)
   print('sending move, socket:', s, 'req:', req)
   s.send(req)
   response = s.recv(1024)
   print(response)
   parsed = parse_response(response)
   print('move player response:', parsed)

def move(pos_delta, match_id):
    print(pos_delta)
    move_player(pos_delta, match_id)

def start_game(id, match_id):
   # pygame setup
   pygame.init()
   screen = pygame.display.set_mode((1280, 720))
   font = pygame.font.Font('freesansbold.ttf', 40)
   clock = pygame.time.Clock()
   running = True
   dt = 0

   player_pos = pygame.Vector2(-100, -100) # player should not be visible until server says so
   oponnent_pos = pygame.Vector2(-100, -100) # player should not be visible until server says so
   ball_pos = pygame.Vector2(-100, -100) # ball should not be visible until server says so
   countdown = None

   while running:
      for event in pygame.event.get():
         if event.type == pygame.QUIT:
               running = False

      screen.fill('black')

      pygame.draw.rect(surface=screen, color=(136, 242, 139), rect=(player_pos, (10, 150)))
      pygame.draw.rect(surface=screen, color=(255, 255, 255), rect=(oponnent_pos, (10, 150)))
      pygame.draw.circle(screen, "red", ball_pos, 30)
      if countdown and countdown > 0:
         text = font.render(str(countdown), True, (255, 255, 255))
         screen.blit(text, (1280//2, 60))

      pos_delta = 300 * dt
      moved = False
      
      update = no_updates()
      if update:
         print(update)
         #{'serverPushUpdate': {'gameStateChange': {'id': '452aba90-373e-4233-9fd0-8679d7650c11', 'state': {'player1Pos': {'id': 'f074c830-f203-4dc9-a666-97cd61562ce9', 'position': {'x': 85, 'y': 360}}, 'player2Pos': {'id': '545ce55a-6bec-470e-b9d8-defca91231b7', 'position': {'x': 1195, 'y': 360}}, 'ballPos': {'x': 0, 'y': 0}, 'countdown': 0}}}}
         game_change = update.get('serverPushUpdate').get('gameStateChange')
         if game_change:
            state = game_change['state']
            countdown = state['countdown']
            player1 = state['player1Pos']
            player2 = state['player2Pos']
            ball = state['ballPos']
            me = None
            oponnent = None
            if player1['id'] == id:
               me = player1
               oponnent = player2
            else:
               me = player2
               oponnent = player1
            
            player_pos.y = me['position']['y']
            player_pos.x = me['position']['x']
            oponnent_pos.y = oponnent['position']['y']
            oponnent_pos.x = oponnent['position']['x']
            ball_pos.x = ball['x']
            ball_pos.y = ball['y']



      keys = pygame.key.get_pressed()
      if keys[pygame.K_UP]:
         pos_delta *= -1
         moved=True
      if keys[pygame.K_DOWN]:
         pos_delta *= 1
         moved=True

      if moved:
         move(pos_delta, match_id)

      pygame.display.flip()
      dt = clock.tick(60) / 1000

   pygame.quit()




def main():
   (id, match_id) = enter_match(s)
   start_game(id, match_id)
   return


s = connect(7878) # TODO: change to function params
if __name__ == '__main__':
   main()
      