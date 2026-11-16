function arnis:story/current
scoreboard players operation #last arnis.story = #steps arnis.story
scoreboard players remove #last arnis.story 1
# The finale plays where its person stands on the ground, or at the place if nobody is there.
execute as @e[type=minecraft:villager,tag=arnis.story_npc] if score @s arnis.story = #step arnis.story run tag @s add arnis.story_here
execute if score #step arnis.story = #last arnis.story at @e[tag=arnis.story_here,limit=1] run function arnis:story/finish_at with storage arnis:story cur
execute if score #step arnis.story = #last arnis.story unless entity @e[tag=arnis.story_here] run function arnis:story/finish with storage arnis:story cur
execute as @e[type=minecraft:villager,tag=arnis.story_here] at @s run function arnis:story/leave
kill @e[type=minecraft:villager,tag=arnis.story_npc]
tag @e[tag=arnis.story_here] remove arnis.story_here
scoreboard players add #step arnis.story 1
execute as @a run function arnis:story/catch_up
function arnis:story/bar
execute if score #step arnis.story >= #steps arnis.story run return 0
function arnis:story/current
tellraw @a [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
playsound minecraft:block.note_block.chime master @a ~ ~ ~ 1 1.2
