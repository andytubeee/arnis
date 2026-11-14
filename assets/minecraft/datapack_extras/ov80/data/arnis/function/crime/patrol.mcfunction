# A few officers near everyone, more once they are hunting; each joins where a pedestrian
# walks, so they patrol the mapped streets.
execute store result score #cops arnis.cr if entity @e[type=minecraft:skeleton,tag=arnis.cop,distance=..80]
scoreboard players operation #cap arnis.cr = #calm arnis.cr
execute if score @s arnis.wanted matches 1 run scoreboard players operation #cap arnis.cr = #hunt arnis.cr
execute if score #cops arnis.cr >= #cap arnis.cr run return 0
execute at @e[tag=arnis.walker,distance=24..64,sort=random,limit=1] run function arnis:crime/cop
