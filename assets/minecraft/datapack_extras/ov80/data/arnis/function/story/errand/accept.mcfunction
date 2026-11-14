# The menu's Take the errand: deliver a parcel to a named place, or find something lost.
execute store result score #n arnis.story run data get storage arnis:story errands
scoreboard players remove #n arnis.story 1
execute store result storage arnis:story ei.n int 1 run scoreboard players get #n arnis.story
function arnis:story/errand/pick with storage arnis:story ei
execute as @e[tag=arnis.errand_npc] run data modify entity @s Glowing set value 0b
tag @e[tag=arnis.errand_npc] remove arnis.errand_npc
scoreboard players set #errand arnis.story 2
scoreboard players set #errand_t arnis.story 0
execute store result score #kind arnis.story run random value 1..2
execute if score #kind arnis.story matches 1 run function arnis:story/errand/parcel
execute if score #kind arnis.story matches 2 run function arnis:story/errand/lost
