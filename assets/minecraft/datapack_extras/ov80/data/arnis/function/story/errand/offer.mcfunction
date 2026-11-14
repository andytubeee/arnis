# Someone near a random player waves them over; nobody near, so try again in a while.
scoreboard players set #errand_t arnis.story 0
execute store result score #n arnis.story run data get storage arnis:story errands
execute if score #n arnis.story matches ..0 run return 0
execute as @r[gamemode=!spectator] at @s as @e[tag=arnis.walker,distance=6..24,sort=random,limit=1] run tag @s add arnis.errand_npc
execute unless entity @e[tag=arnis.errand_npc] run return 0
execute as @e[tag=arnis.errand_npc] run data modify entity @s Glowing set value 1b
execute as @e[tag=arnis.errand_npc] at @s run tellraw @a[distance=..32] {"text":"Someone nearby is waving at you. Right-click them to see what they need.","color":"aqua"}
scoreboard players set #errand arnis.story 1
