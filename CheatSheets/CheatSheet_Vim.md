# Navigation
* h : Move left
* j : Move down
* k : Move up
* l : Move right
* <n>j : Move n lines down
* H : Top line of the screen
* M : Middle line of the screen
* L : Bottom line of the screen
* gg : First line of file
* G : Last line of file
* :<n> : Line n of file
* e : End of current word
* b : Beginning of current word
* v : Beginning of next word
* 0 : Beggining of line
* ^ : First non whitespace character of line
* $ : End of current line
	
# Files
* w : Save the current file
* wq : Save and close
* W <name> : Save a copy of the current file as name but continue editing current one

# Modes
* i : Insert mode
* : : Command mode
* R : Replace mode
* v : Visual mode (highlighting)
* V : Visual line mode
* Escape : Return to normal mode from insert or replace mode
* Escape + escape : Return to normal mode from command or visual mode

# Edition
* i : Insert before current character
* a : Insert after current character
* I : Insert at the first non-whitespace character of the line
* o : Insert a line below the current line, then enter insert mode
* O : Insert a line above the current line, then enter insert mode
* r : Overwrite one character and return to command mode
* U : Undo
* Ctrl + R : Redo

# Copy Paste
* Changes apply to highlighted text in visual mode
		○ C : Change the rest of the current line
		○ c : Cut and enter insert mode
		○ d : Delete and cut
		○ D : Delete rest of current line
		○ p : paste after cursor
		○ P : Paste before cursor
		○ x : Delete after the cursor
		○ X : Delete before the cursor
* Change apply to following command in normal mode
		○ cw : Change one word
		○ c4w : Change 4 words
		○ c4l : Change 4 letters
		○ cc : Change current line
		○ 4x : Change four characters after the ursor
		○ 4p : Paste four times after the cursor
* :set paste : enter paste mode
* :set nopaste : exit paste mode
* Ctrl + Shft + V : Paste
