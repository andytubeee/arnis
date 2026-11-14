# The pause page (1 where next, 2 skip, 3 errand) and a step's own button (10 + its number),
# which counts only beside its person, so a stale or second press moves nothing on.
execute if score @s arnis.story.ctl matches 1 run function arnis:story/where
execute if score @s arnis.story.ctl matches 2 run function arnis:story/skip
execute if score @s arnis.story.ctl matches 3 run function arnis:story/errand/info
scoreboard players operation #want arnis.story = #step arnis.story
scoreboard players add #want arnis.story 10
execute as @e[type=minecraft:villager,tag=arnis.story_npc,distance=..8] if score @s arnis.story = #step arnis.story run tag @s add arnis.story_here
execute if score @s arnis.story.ctl = #want arnis.story if entity @e[tag=arnis.story_here] run function arnis:story/advance
tag @e[tag=arnis.story_here] remove arnis.story_here
scoreboard players reset @s arnis.story.ctl
