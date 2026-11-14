execute as @a run function arnis:crime/player
scoreboard players add #cr_t arnis.cr 1
execute if score #cr_t arnis.cr matches 20.. run function arnis:crime/second
