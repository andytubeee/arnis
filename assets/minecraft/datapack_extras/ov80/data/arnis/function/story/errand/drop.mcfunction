execute as @e[tag=arnis.errand_npc] run data modify entity @s Glowing set value 0b
tag @e[tag=arnis.errand_npc] remove arnis.errand_npc
kill @e[type=minecraft:item,tag=arnis.lost]
clear @a minecraft:paper[minecraft:custom_data={arnis_parcel:1b}]
clear @a minecraft:leather_helmet[minecraft:custom_data={arnis_lost:1b}]
function arnis:story/errand/reset
