scoreboard players set #story_t arnis.story 0
bossbar set arnis:story players @a
execute as @a unless score @s arnis.story.seen matches 0.. run function arnis:story/intro
function arnis:story/errand/second
execute if score #step arnis.story >= #steps arnis.story run return run function arnis:story/between
function arnis:story/current
execute if score #step arnis.story = #hunt_step arnis.story if score #hunt arnis.story matches 1.. run function arnis:story/hunt/second
execute if data storage arnis:story cur{kind:"riddle"} run return run function arnis:story/beat/riddle with storage arnis:story cur
execute if data storage arnis:story cur{kind:"search"} run return run function arnis:story/beat/search with storage arnis:story cur
# Whoever waits at the current stop is there while a player is near, glowing to be found.
execute as @e[type=minecraft:villager,tag=arnis.story_npc] unless score @s arnis.story = #step arnis.story run kill @s
execute unless entity @e[type=minecraft:villager,tag=arnis.story_npc] run function arnis:story/spawn with storage arnis:story cur
execute as @e[type=minecraft:villager,tag=arnis.story_npc] at @s unless entity @a[distance=..64] run kill @s
