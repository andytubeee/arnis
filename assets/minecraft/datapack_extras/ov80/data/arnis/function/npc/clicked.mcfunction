data modify storage arnis:npc who set from storage arnis:npc me
execute store success score #other arnis.npc run data modify storage arnis:npc who set from entity @s interaction.player
execute if score #other arnis.npc matches 1 run return 0
data remove entity @s interaction
scoreboard players operation #k arnis.npc = @s arnis.wk
execute as @a[tag=arnis.npc_me,limit=1] at @s run function arnis:npc/open
