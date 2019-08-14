import random

class Game(object):
	"""the class wihch handles all operations and teh game continuity"""
	def __init__(self, rows=4, columns=4, initial_tiles=2):
		self.rows=rows
		self.columns=columns
		self.initial_tiles=2
		#making the according matrix
		self.make_board()
		
	def get_row(self, row):
		row_members = []
		count = 0
		for cell in self.matrix:
			if count//4 == row:
				row_members.append([count, cell])

			count+=1

		return row_members


	def get_column(self, col):
		col_members = []
		count = 0

		for cell in self.matrix:
			if (count - 4*(count//4)) == col:
				col_members.append([count, cell])
			count+=1

		return col_members


	def make_board(self):
		self.matrix = []
		for  i in range(self.rows*self.columns):
			self.matrix.append(0)


	def get_free(self):
		count = 0
		free = []
		for cell in self.matrix:
			if cell==0:
				free.append(count)

			count+=1

		return free

	def populate(self):
		to_choose = self.get_free()
		cell = random.choice(to_choose)

		#may wish to change this static probability so a more evolutive and pondered one
		prob = []
		for j in range(9):prob.append(2)
		for j in range(1):prob.append(4)
		value = random.choice(prob)

		self.matrix[cell]=value

		return (cell, value)

	def get_score(self):
		'''returns an int represnting the current score based upon the present tiles'''
		score= 0
		for cell in self.matrix:
			score+=cell
		return score


	def move(self, side):
		'''moves all of the tiles to a side.
		0 is left
		1 is right
		2 is up
		3 is down'''

		rc = []

		if side<2:
			for row in range(self.rows):
				rc.append(self.get_row(row))
			if side==1:
				for row in rc:
					row.reverse()


		else:
			for col in range(self.columns):
				c = self.get_column(col)
				rc.append(c)
			if side==3:
				for col in rc:
					col.reverse()

		#print(rc, "\n")
		has_merged=[]
		for line in rc:
			line_stacked = False
			while not line_stacked:
				line_stacked = True
				for cell, idx in zip(line, range(len(line)-1)):
					if cell[1]==0 and line[idx+1][1]!=0:
						cell[1] = line[idx+1][1]
						line[idx+1][1] = 0
						line_stacked=False

					#merging
					elif cell[1]==line[idx+1][1] and cell[1]!=0 and cell[0] not in has_merged:
						cell[1]+=line[idx+1][1]
						line[idx+1][1] = 0
						line_stacked=False
						if side<2:
							has_merged.append(cell[0]-1)
						else:
							has_merged.append(cell[0]+4)


		for line in rc:
			for cell in line:
				self.matrix[cell[0]] = cell[1]

	def won(self):
		for cell in self.matrix:
			if cell>=2048: return True
		return False