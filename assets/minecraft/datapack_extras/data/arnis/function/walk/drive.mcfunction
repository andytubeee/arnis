scoreboard players set #walk_t arnis 0
# Only walkers someone could see take a step: the game draws a person-sized mob out to
# about 64 blocks, so farther ones stand still until a player comes closer.
execute as @e[tag=arnis.walker] at @s if entity @a[distance=..64] run function arnis:walk/step
scoreboard players add #walk_r arnis 1
execute if score #walk_r arnis matches 10.. run function arnis:walk/respawn
