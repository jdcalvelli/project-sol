# TURBO NARRATIVE DSL README - name tbd

### `#` starts a comment

- keep in mind: the dsl is read at game runtime, so comments are not compiled out

- this means that comments can, at present, affect execution time, so use them intentionally

### `<< [name]` is the name of a passage

### `>> [name]` is a send to the passage of corresponding name

- sends can live on their own, or in conjunction with a choice

### `]> [choice]` denotes a choice, the dsl expects two choices at a time

- choice texts that follow ]> are written out to text boxe

- a choice must be followed by two diverts on the next line, in the order that corresponds to the order of choices

- e.g. the first divert corresponds to choice one, the second to choice two

### `-- end` denotes end of passage

- once the script game hits an -- end block, the game ends

- so all passages must divert somewhere, or end the game

### `[char]:[text]` lines are statements

- currently, assumption is two characters, LEFT or RIGHT

- char determines portrait display

- text is written out to text boxes

### blank lines are ignored

- keep in mind: the dsl is read at game runtime, so blank lines are not compiled out
