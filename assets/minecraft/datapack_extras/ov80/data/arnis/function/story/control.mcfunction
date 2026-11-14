# The pause page (1 where next, 2 skip, 3 errand) and a step's own button (10 + its number),
# which counts only beside its person, so a stale or second press moves nothing on.
execute if score @s arnis.story.ctl matches 1 run function arnis:story/where
execute if score @s arnis.story.ctl matches 2 run function arnis:story/skip
execute if score @s arnis.story.ctl matches 3 run function arnis:story/errand/info
scoreboard players operation #want arnis.story = #step arnis.story
scoreboard players add #want arnis.story 10
execute if score @s arnis.story.ctl = #want arnis.story if entity @e[type=minecraft:villager,tag=arnis.story_npc,distance=..8] run function arnis:story/advance
scoreboard players reset @s arnis.story.ctl
