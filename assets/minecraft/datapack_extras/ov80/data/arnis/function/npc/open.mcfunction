# The menu for walker #k: their name and trait, what anyone can do, and what the modes add.
execute as @e[tag=arnis.person,distance=..8] if score @s arnis.wk = #k arnis.npc run tag @s add arnis.npc_it
execute unless entity @e[tag=arnis.npc_it,distance=..8] run return 0
scoreboard players operation @s arnis.npc.t = #k arnis.npc
scoreboard players reset @s arnis.npc
scoreboard players enable @s arnis.npc
# They stop to talk, for as long as a menu takes.
scoreboard players set @e[tag=arnis.npc_it,distance=..8] arnis.ww 60
data modify storage arnis:npc menu.title set value {text:"Someone"}
data modify storage arnis:npc menu.title set from entity @e[tag=arnis.npc_it,distance=..8,limit=1] CustomName
execute store result storage arnis:npc menu.i int 1 run scoreboard players get @e[tag=arnis.npc_it,distance=..8,limit=1] arnis.npc.tr
function arnis:npc/trait with storage arnis:npc menu
data modify storage arnis:npc menu.buttons set value []
execute if entity @e[tag=arnis.npc_it,tag=arnis.story_npc,distance=..8] run data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.talk
execute if entity @e[tag=arnis.npc_it,tag=arnis.errand_npc,distance=..8] run data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.errand
data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.hi
data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.chat
data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.directions
execute if score #crime arnis.npc matches 1 unless entity @e[tag=arnis.npc_it,tag=arnis.story_npc,distance=..8] run data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.rob
data modify storage arnis:npc menu.buttons append from storage arnis:npc buttons.bye
function arnis:npc/show with storage arnis:npc menu
tag @e[tag=arnis.npc_it] remove arnis.npc_it
