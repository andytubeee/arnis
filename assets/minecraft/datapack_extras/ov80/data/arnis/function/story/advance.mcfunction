scoreboard players set #moved arnis.story 1
function arnis:story/hunt/stop
function arnis:story/current
scoreboard players operation #last arnis.story = #steps arnis.story
scoreboard players remove #last arnis.story 1
# A right answer's reply, which says where to go next.
execute if data storage arnis:story cur{kind:"choice"} run tellraw @a [{"storage":"arnis:story","nbt":"cur.npc","color":"aqua"},{"text":": ","color":"aqua"},{"storage":"arnis:story","nbt":"cur.after","color":"white"}]
# The finale plays where its person stands on the ground, or at the place if nobody is there.
execute as @e[type=minecraft:villager,tag=arnis.story_npc] if score @s arnis.story = #sid arnis.story run tag @s add arnis.story_here
execute if score #step arnis.story = #last arnis.story at @e[tag=arnis.story_here,limit=1] run function arnis:story/finish_at with storage arnis:story cur
execute if score #step arnis.story = #last arnis.story unless entity @e[tag=arnis.story_here] run function arnis:story/finish with storage arnis:story cur
execute as @e[type=minecraft:villager,tag=arnis.story_here] at @s run function arnis:story/leave
kill @e[type=minecraft:villager,tag=arnis.story_npc]
tag @e[tag=arnis.story_here] remove arnis.story_here
kill @e[type=minecraft:item,tag=arnis.search]
clear @a *[minecraft:custom_data~{arnis_search:1b}]
scoreboard players add #step arnis.story 1
scoreboard players set #beat_t arnis.story 0
execute if score #step arnis.story >= #steps arnis.story run return run function arnis:story/done
function arnis:story/bar
function arnis:story/current
function arnis:story/next_stop
execute as @a at @s run playsound minecraft:block.note_block.chime master @s ~ ~ ~ 1 1.2
