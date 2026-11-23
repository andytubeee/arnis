# A riddle: nobody waits there, and reaching the place solves it.
kill @e[type=minecraft:villager,tag=arnis.story_npc]
scoreboard players add #beat_t arnis.story 1
function arnis:story/bar
$execute positioned $(x) $(y) $(z) if entity @a[gamemode=!spectator,distance=..14] run function arnis:story/beat/found
