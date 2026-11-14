scoreboard players set @s arnis.sneak 0
tag @e[tag=arnis.walker,tag=!arnis.robbed,distance=..2.5,sort=nearest,limit=1] add arnis.robbed
execute store result score #take arnis.cr run random value 5..60
scoreboard players operation @s arnis.cash += #take arnis.cr
tellraw @s [{"text":"You robbed a pedestrian: +$","color":"green"},{"score":{"name":"#take","objective":"arnis.cr"},"color":"green"}]
playsound minecraft:entity.item.pickup player @s
function arnis:crime/sight
execute if score #sighted arnis.cr matches 1 run function arnis:crime/spotted
