# A wrong answer gets a nudge, nothing more.
function arnis:story/current
tellraw @s [{"storage":"arnis:story","nbt":"cur.npc","color":"aqua"},{"text":": ","color":"aqua"},{"storage":"arnis:story","nbt":"cur.wrong","color":"white"}]
playsound minecraft:block.note_block.bass master @s ~ ~ ~ 0.6 0.8
