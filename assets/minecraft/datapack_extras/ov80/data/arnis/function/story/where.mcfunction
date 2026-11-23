execute if score #step arnis.story >= #steps arnis.story run return run tellraw @s {"text":"That story's done. The next one starts in a moment.","color":"gray"}
function arnis:story/current
execute if data storage arnis:story cur{kind:"riddle"} run return run tellraw @s [{"text":"Riddle: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.hint","color":"yellow"},{"text":". That one's for you to work out!","color":"gray"}]
function arnis:story/next_place
function arnis:npc/hint
tellraw @s [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:npc","nbt":"where.name","color":"yellow"},{"text":", about ","color":"gray"},{"score":{"name":"#m","objective":"arnis.npc"},"color":"white"},{"text":" m ","color":"white"},{"storage":"arnis:npc","nbt":"way","color":"white"},{"text":". Look for the glowing person.","color":"gray"}]
