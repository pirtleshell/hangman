# HangmanCanvas

this isn't actually a hangman game. it's a cli that will print ascii drawings of steps of a hangman game!

is it overkill? probably. would it have been easier to just make ascii slides that get printed? definitely! but was it a fun learning experience to design and write? you bet!


## how it works

the canvas divides a game of hangman into 7 steps (0 to 6). the step number represents the number of missed guesses. there are two methods you should know exist:

`canvas.print_step(n: usize)`: prints the ascii drawing for the `n`th step of the game

`canvas.loss(n: usize)`: same as above, but also notifies the user of which body part was lost.


## under the hood

there are two ascii files that make the magic happen. [one](src/hangman.ascii) is the completed drawing of a lost game. [the other](src/hangman_mask.ascii) is a mask of which corresponding characters of the completed ascii break into which reveal steps. with this setup, the actual ascii image gets revealed can be customized with ascii drawing of even higher calibers than the one i made in 5 minutes.

# license

© 2020, [Robert Pirtle](https://robert.pirtle.xyz/). licensed under [MIT](https://choosealicense.com/licenses/mit/).
