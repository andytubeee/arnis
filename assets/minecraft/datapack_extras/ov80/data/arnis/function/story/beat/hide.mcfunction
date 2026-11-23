# Somewhere within ten blocks of the place, on the ground the way people stand there.
$summon minecraft:item $(x) $(y) $(z) {Tags:["arnis.search","arnis.search_new"],Invulnerable:1b,PickupDelay:0,Age:-32768,Item:{id:"$(item)",count:1,components:{"minecraft:custom_name":{text:"$(thing)",color:"gold",italic:false},"minecraft:custom_data":{arnis_search:1b}}}}
$execute store success score #ok arnis.story run spreadplayers $(x) $(z) 0 10 under $(sy) false @e[type=minecraft:item,tag=arnis.search_new]
$execute if score #ok arnis.story matches 0 run spreadplayers $(x) $(z) 0 10 false @e[type=minecraft:item,tag=arnis.search_new]
tag @e[tag=arnis.search_new] remove arnis.search_new
