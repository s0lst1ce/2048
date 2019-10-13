import os
import pygame as pg
from settings import *
from game import Game
import pigui as pig
from pigui.colors import *
#GLOBALS
running = False
playing = False
tilew = int((WIDTH-((COLUMNS-1)*SEP_W))/COLUMNS) #tile placement may benefit from floats
tileh = int((HEIGHT-((ROWS-1)*SEP_H))/ROWS)

#display
pg.init()
window = pg.display.set_mode((WIDTH, HEIGHT))
pg.display.set_caption("A 2048 clone by s0lst1ce")
clock = pg.time.Clock()

#GUI
main_menu = pig.Container(0, 0, WIDTH, HEIGHT, visible=False)

score_box = pig.Container(WIDTH - SCORE_W, 10, SCORE_W, SCORE_H, visible=True)
score = pig.Label(SCORE_W, SCORE_H, text="000000", fgcolor=BLACK)
tb = pig.TextButton(100,50,text="Hello", fgcolor=GREEN, action=lambda:print("hey!"))
score_box.add(score, 0, 0, ch=100, cw=100)
main_menu.add(tb, 0,0)
print(score_box)

#sound
pg.mixer.init()
musics =[]
for music in os.listdir(os.path.join("audio")):
	musics.append(pg.mixer.music.load(os.path.join("audio", music)))
	#pg.mixer.music.queue(musics[-1])

#SPRITES
def load_sprites():
	'''returns a dict containing all tiles' sprites surfaces'''
	surfs = {0:pg.Surface((tilew, tileh))}
	surfs[0].fill(WHITE)
	for file in os.listdir(os.path.join("sprites")):
		surf=pg.image.load(os.path.join("sprites", file)).convert_alpha()
		surfs[int(file.split(".")[0])]=pg.transform.scale(surf, (tilew, tileh))
	return surfs
sprites = load_sprites()

#SETUP
def start():
	'''inits and starts the game'''
	global running
	global playing
	running=True
	playing=True
	g = Game(rows=ROWS, columns=COLUMNS)
	g.make_board()
	for c in range(g.initial_tiles):
		g.populate()
	return g

g = start()
score.text=str(g.get_score())
score_box.visible=True


#GAME LOGIC
def events():
	'''processes events'''
	global running
	for event in pg.event.get():
		if event.type == pg.QUIT:
			running=False

		if event.type == pg.KEYUP:
			if event.key==pg.K_ESCAPE:
				running = False

			#handling movement
			old_matrix = g.matrix.copy()
			if event.key==pg.K_LEFT:
				g.move(0)
			if event.key==pg.K_RIGHT:
				g.move(1)
			if event.key==pg.K_UP:
				g.move(2)
			if event.key==pg.K_DOWN:
				g.move(3)

			if g.matrix!=old_matrix:
				g.populate()
			score.text=str(g.get_score())

		#loop music
		if not pg.mixer.music.get_busy():
			pg.mixer.music.rewind()

def update():
	'''ran each tick handles all modification based on occured events'''
	#gui update
	score_box.update()
	#game update
	if playing:
		game_update()
	
def game_update():
	'''updates the game (ie: not the GUI elements)'''
	if len(g.get_free())==0:
		running = False

def render():
	'''handles the rendering'''
	window.fill(WHITE)
	col = 0
	for r in range(ROWS):
		for cell, ci in zip(g.get_row(r), range(COLUMNS)):
			window.blit(sprites[cell[1]], (ci*(tilew+SEP_W), r*(tileh+SEP_H)))

		col+=1

	score_box.draw(window)
	main_menu.draw(window)
	pg.display.flip()


def main_loop():
	'''main game logic handler'''
	pg.mixer.music.play(-1)
	while running:
		clock.tick()
		events()
		update()
		render()
	print(f'''You won: {g.won()} with {g.get_score()}''')

main_loop()