function arnis:story/current
tellraw @s [{"selector":"@e[tag=arnis.npc_it,limit=1]","color":"aqua"},{"text":": ","color":"aqua"},{"text":"Have you heard? Something's going on over at ","color":"white"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"},{"text":".","color":"white"}]
