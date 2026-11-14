# The menu's choice, the walker each player last opened it on, and each walker's trait.
scoreboard objectives add arnis.npc trigger
scoreboard objectives add arnis.npc.t dummy
scoreboard objectives add arnis.npc.tr dummy
scoreboard players set #npc_t arnis.npc 0
scoreboard players set #2 arnis.npc 2
scoreboard players set #-1 arnis.npc -1
function arnis:npc/flags
function arnis:npc/lines
