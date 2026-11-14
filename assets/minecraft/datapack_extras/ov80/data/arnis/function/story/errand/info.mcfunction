execute if score #errand arnis.story matches 0 run return run tellraw @s {"text":"No errand right now. Someone will wave you over before long.","color":"gray"}
execute if score #errand arnis.story matches 1 run return run tellraw @s {"text":"Someone nearby is waving at you: look for the glowing person.","color":"aqua"}
execute if score #kind arnis.story matches 2 run return run tellraw @s {"text":"Find the lost hat: it glows, and it isn't far from where you took the errand.","color":"aqua"}
data modify storage arnis:npc where set value {}
data modify storage arnis:npc where.name set from storage arnis:story errand.to
data modify storage arnis:npc where.x set from storage arnis:story errand.tx
data modify storage arnis:npc where.z set from storage arnis:story errand.tz
function arnis:npc/hint
tellraw @s [{"text":"Deliver the parcel to ","color":"aqua"},{"storage":"arnis:npc","nbt":"where.name","color":"yellow"},{"text":", about ","color":"gray"},{"score":{"name":"#m","objective":"arnis.npc"},"color":"white"},{"text":" m ","color":"white"},{"storage":"arnis:npc","nbt":"way","color":"white"},{"text":".","color":"gray"}]
