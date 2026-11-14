scoreboard players set #story_t arnis.story 0
bossbar set arnis:story players @a
execute as @a unless score @s arnis.story.seen matches 0.. run function arnis:story/intro
execute as @a run function arnis:story/catch_up
function arnis:story/errand/second
execute if score #step arnis.story >= #steps arnis.story run return 0
# Whoever waits at the current stop is there while a player is near, glowing to be found.
function arnis:story/current
execute unless entity @e[type=minecraft:villager,tag=arnis.story_npc] run function arnis:story/spawn with storage arnis:story cur
execute as @e[type=minecraft:villager,tag=arnis.story_npc] at @s unless entity @a[distance=..64] run kill @s
