# Now and then, with none around, an arms dealer sets up on a street near this player: a
# step off a pedestrian's lane towards the player, on open ground, so nobody walks through.
execute if entity @e[type=minecraft:wandering_trader,tag=arnis.dealer,distance=..80] run return 0
execute store result score #roll arnis.cr run random value 1..30
execute if score #roll arnis.cr matches 1 at @e[tag=arnis.walker,distance=16..48,sort=random,limit=1] facing entity @s feet positioned ^ ^ ^1.5 if block ~ ~ ~ #minecraft:replaceable if block ~ ~1 ~ #minecraft:replaceable unless block ~ ~-1 ~ #minecraft:replaceable run function arnis:crime/dealer_spawn
