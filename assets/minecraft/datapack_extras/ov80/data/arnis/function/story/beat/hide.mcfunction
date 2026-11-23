# At the place's nook, a few steps off the street in the open (worked out at generation), on
# the ground there.
$summon minecraft:item $(hx) $(y) $(hz) {Tags:["arnis.search","arnis.search_new"],Invulnerable:1b,PickupDelay:0,Age:-32768,Item:{id:"$(item)",count:1,components:{"minecraft:custom_name":{text:"$(thing)",color:"gold",italic:false},"minecraft:custom_data":{arnis_search:1b,sid:$(sid)}}}}
$execute store success score #ok arnis.story run spreadplayers $(hx) $(hz) 0 1 under $(sy) false @e[type=minecraft:item,tag=arnis.search_new]
$execute if score #ok arnis.story matches 0 run spreadplayers $(hx) $(hz) 0 1 false @e[type=minecraft:item,tag=arnis.search_new]
tag @e[tag=arnis.search_new] remove arnis.search_new
