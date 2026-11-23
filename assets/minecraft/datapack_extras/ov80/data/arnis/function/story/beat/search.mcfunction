# A search: the thing lies unmarked near the place once someone is close, and counts once
# anyone holds it.
kill @e[type=minecraft:villager,tag=arnis.story_npc]
execute as @a if items entity @s container.* *[minecraft:custom_data~{arnis_search:1b}] at @s run return run function arnis:story/beat/found
execute as @a if items entity @s weapon.offhand *[minecraft:custom_data~{arnis_search:1b}] at @s run return run function arnis:story/beat/found
$execute positioned $(x) $(y) $(z) if entity @a[distance=..48] if loaded ~ ~ ~ unless entity @e[type=minecraft:item,tag=arnis.search,distance=..64] run function arnis:story/beat/hide with storage arnis:story cur
execute at @e[type=minecraft:item,tag=arnis.search] run particle minecraft:wax_off ~ ~0.3 ~ 0.2 0.1 0.2 0 2
data modify storage arnis:story heat set value "cold"
execute at @e[type=minecraft:item,tag=arnis.search] if entity @a[gamemode=!spectator,distance=..24] run data modify storage arnis:story heat set value "warm"
execute at @e[type=minecraft:item,tag=arnis.search] if entity @a[gamemode=!spectator,distance=..10] run data modify storage arnis:story heat set value "hot!"
execute at @e[type=minecraft:item,tag=arnis.search] if entity @a[gamemode=!spectator,distance=..4] run data modify storage arnis:story heat set value "right here!"
function arnis:story/bar
