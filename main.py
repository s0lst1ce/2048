from game import *

class TerminalInterface(Game):
	"""enables one to issue its move orders to the game. This class provides bindings
	for all actions and will see that they get doen properly."""
	def __init__(self, rows=4, columns=4, initial_tiles=2):
		super(TerminalInterface, self).__init__(rows, columns, initial_tiles)
		self.mvts = {
		0: ["left", "l", "4"],
		1: ["right", "r", "6"],
		2: ["up", "u", "8"],
		3: ["down", "d", "2"]
		}

	def show_board(self):
		'''only exists for testing purposes'''
		cells = []
		for cell in self.matrix:
			p_cell=str(cell)
			blanck_to_add = 3-len(p_cell)
			for i in range(blanck_to_add):
				p_cell=" "+p_cell
			cells.append(p_cell)

		print('''{0[0]} | {0[1]} | {0[2]} | {0[3]}\n-------------------------\n{0[4]} | {0[5]} | {0[6]} | {0[7]}\n-------------------------\n{0[8]} | {0[9]} | {0[10]} | {0[11]}\n-------------------------\n{0[12]} | {0[13]} | {0[14]} | {0[15]}\n'''.format(cells))


	def main_loop(self):
		'''at its core a while loop which will wait for the player to input a command from the console.
		if an incorrect command is given teh user will receive a message
		allowed commands are self.mvts dict values'''
		for i in range(self.initial_tiles):
			self.populate()
		running = True
		while running:
			self.show_board()
			mv = input(">").lower()
			if mv == "q":
				running=False

			else:
				found = False
				for k, v in self.mvts.items():
					if mv in v:
						found = True
						bck_mtx = self.matrix.copy()
						self.move(k)
						if bck_mtx != self.matrix:
							self.populate()
						else:
							if self.get_free()==0:
								running=False

						break

				if not found:
					print("Unrecognized command")

		score = self.get_score()
		if self.won():
			print("You won ! Good job ! Now that you've mastered the fundamentals why don't you try out new goals ? Try to reach higher numbers and if you have trouble, augment the size of the board.")
		else:
			print("You lost ! Will your next try give a 2048 ?")
		quit()



g=TerminalInterface()
g.main_loop()