# A few officers near everyone, more for each wanted star; each joins where a pedestrian
# walks, so they patrol the mapped streets. From four stars, new officers are SWAT.
execute store result score #cops arnis.cr if entity @e[type=minecraft:skeleton,tag=arnis.cop,distance=..80]
scoreboard players operation #cap arnis.cr = #calm arnis.cr
execute if score @s arnis.wanted matches 1.. run function arnis:crime/posse
execute if score #cops arnis.cr >= #cap arnis.cr run return 0
execute store success score #swat arnis.cr if score @s arnis.wanted matches 4..
execute at @e[tag=arnis.walker,distance=24..64,sort=random,limit=1] run function arnis:crime/cop
