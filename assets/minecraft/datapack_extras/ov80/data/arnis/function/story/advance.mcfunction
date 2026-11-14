kill @e[type=minecraft:villager,tag=arnis.story_npc]
function arnis:story/current
scoreboard players operation #last arnis.story = #steps arnis.story
scoreboard players remove #last arnis.story 1
execute if score #step arnis.story = #last arnis.story run function arnis:story/finish with storage arnis:story cur
scoreboard players add #step arnis.story 1
execute as @a run function arnis:story/catch_up
function arnis:story/bar
execute if score #step arnis.story >= #steps arnis.story run return 0
function arnis:story/current
tellraw @a [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
playsound minecraft:block.note_block.chime master @a ~ ~ ~ 1 1.2
