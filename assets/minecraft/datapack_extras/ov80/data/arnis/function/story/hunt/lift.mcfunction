# The heavy thing: two players beside it lift it free; one alone is told to bring a friend.
execute if score #duo arnis.story matches 0 run return run function arnis:story/hunt/light
execute store result score #n arnis.story if entity @a[gamemode=!spectator,distance=..4]
execute if score #n arnis.story matches 2.. run return run function arnis:story/hunt/light
# Alone beside it for half a minute, they manage after all.
execute if entity @a[gamemode=!spectator,distance=..4] run scoreboard players add @s arnis.story 1
execute if score @s arnis.story matches 30.. run return run function arnis:story/hunt/light
execute as @a[gamemode=!spectator,distance=..4,tag=!arnis.told] run tellraw @s {"text":"It won't budge on your own. Bring a friend and lift it together!","color":"aqua"}
tag @a[gamemode=!spectator,distance=..4] add arnis.told
particle minecraft:crit ~ ~0.5 ~ 0.3 0.2 0.3 0.05 4
