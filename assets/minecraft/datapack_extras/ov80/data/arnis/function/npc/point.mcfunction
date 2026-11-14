# "<place> is about <m> m <way>", for arnis:npc where, from this player.
function arnis:npc/hint
tellraw @s [{"selector":"@e[tag=arnis.npc_it,limit=1]","color":"aqua"},{"text":": ","color":"aqua"},{"storage":"arnis:npc","nbt":"where.name","color":"white"},{"text":"? That's about ","color":"white"},{"score":{"name":"#m","objective":"arnis.npc"},"color":"yellow"},{"text":" m ","color":"yellow"},{"storage":"arnis:npc","nbt":"way","color":"yellow"},{"text":" from here.","color":"white"}]
