execute if score #step arnis.story >= #steps arnis.story run return run tellraw @s {"text":"The story is over. Passers-by still have errands.","color":"gray"}
function arnis:story/next_place
function arnis:npc/hint
tellraw @s [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:npc","nbt":"where.name","color":"yellow"},{"text":", about ","color":"gray"},{"score":{"name":"#m","objective":"arnis.npc"},"color":"white"},{"text":" m ","color":"white"},{"storage":"arnis:npc","nbt":"way","color":"white"},{"text":". Look for the glowing person.","color":"gray"}]
