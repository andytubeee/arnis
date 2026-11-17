# Each second of the hunt: a job for anyone without one, each thing out near its place,
# a shimmer over it to spot from down the street, and the bars.
execute as @a[gamemode=!spectator] at @s run function arnis:story/hunt/assign
execute if score #f0 arnis.story matches 0 run function arnis:story/hunt/thing with storage arnis:story hunt[0]
execute if score #f1 arnis.story matches 0 run function arnis:story/hunt/thing with storage arnis:story hunt[1]
execute if score #f2 arnis.story matches 0 run function arnis:story/hunt/thing with storage arnis:story hunt[2]
# Friends who leave mid-hunt never leave someone alone with a two-person lift.
execute store result score #n arnis.story if entity @a[gamemode=!spectator]
execute if score #n arnis.story matches ..1 run scoreboard players set #duo arnis.story 0
execute as @e[type=minecraft:item,tag=arnis.heavy] at @s run function arnis:story/hunt/lift
execute at @e[type=minecraft:item,tag=arnis.hunt] run particle minecraft:end_rod ~ ~2 ~ 0.1 2 0.1 0.01 6 force
execute if score #f0 arnis.story matches 1 if score #f1 arnis.story matches 1 if score #f2 arnis.story matches 1 run return run function arnis:story/hunt/found
bossbar set arnis:hunt0 players @a[scores={arnis.story.job=0}]
bossbar set arnis:hunt1 players @a[scores={arnis.story.job=1}]
bossbar set arnis:hunt2 players @a[scores={arnis.story.job=2}]
execute as @a[scores={arnis.story.job=0},limit=1] run function arnis:story/hunt/bar {k:0}
execute as @a[scores={arnis.story.job=1},limit=1] run function arnis:story/hunt/bar {k:1}
execute as @a[scores={arnis.story.job=2},limit=1] run function arnis:story/hunt/bar {k:2}
function arnis:story/bar
