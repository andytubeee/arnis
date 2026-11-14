# The menu's Rob, as the player, on the walker it was opened on (arnis.npc_it).
execute if entity @e[tag=arnis.npc_it,tag=arnis.robbed,distance=..4] run return run tellraw @s {"text":"They've got nothing left.","color":"gray","italic":true}
tag @e[tag=arnis.npc_it,distance=..4] add arnis.robbed
# Cash is emeralds, $10 each: carried, spent at arms dealers and dropped on death.
execute store result score #take arnis.cr run random value 1..6
execute store result storage arnis:crime take.n int 1 run scoreboard players get #take arnis.cr
function arnis:crime/pay with storage arnis:crime take
scoreboard players operation #take arnis.cr *= #10 arnis.cr
tellraw @s [{"text":"You robbed a pedestrian: +$","color":"green"},{"score":{"name":"#take","objective":"arnis.cr"},"color":"green"}]
playsound minecraft:entity.item.pickup player @s
playsound minecraft:entity.villager.no neutral @a[distance=..16]
function arnis:crime/sight
execute if score #sighted arnis.cr matches 1 run return run function arnis:crime/spotted
# Unseen by the police, a second pedestrian nearby (the victim is the first) calls them half the time.
execute store result score #near arnis.cr if entity @e[tag=arnis.walker,distance=..10]
execute if score #near arnis.cr matches ..1 run return 0
execute store result score #roll arnis.cr run random value 1..2
execute if score #roll arnis.cr matches 1 run function arnis:crime/reported
