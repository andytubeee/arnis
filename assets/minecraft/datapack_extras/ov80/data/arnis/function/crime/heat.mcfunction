# Seen by an officer resets the chase. #delay seconds out of sight starts the #countdown,
# and the police stand down when it runs out.
# A two-tone siren from the nearest officer while one is close.
execute if score #siren arnis.cr matches 0 at @e[type=minecraft:skeleton,tag=arnis.cop,distance=..32,sort=nearest,limit=1] run playsound minecraft:block.note_block.bit hostile @s ~ ~ ~ 2 0.8
execute if score #siren arnis.cr matches 1 at @e[type=minecraft:skeleton,tag=arnis.cop,distance=..32,sort=nearest,limit=1] run playsound minecraft:block.note_block.bit hostile @s ~ ~ ~ 2 1.2
function arnis:crime/sight
execute if score #sighted arnis.cr matches 1 run return run function arnis:crime/sighted
scoreboard players add @s arnis.unseen 1
execute if score @s arnis.unseen <= #delay arnis.cr run return 0
scoreboard players remove @s arnis.hide 1
execute if score @s arnis.hide matches ..0 run function arnis:crime/clear
