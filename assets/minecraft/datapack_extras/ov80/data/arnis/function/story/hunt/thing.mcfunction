# One thing of the hunt: found once anyone holds it, else out at its place while someone
# is near, on the ground the way people stand there.
$execute as @a if items entity @s container.* *[minecraft:custom_data~{arnis_hunt:1b,thing:$(k)}] run tag @s add arnis.finder
$execute as @a if items entity @s weapon.offhand *[minecraft:custom_data~{arnis_hunt:1b,thing:$(k)}] run tag @s add arnis.finder
$execute if entity @a[tag=arnis.finder] run return run function arnis:story/hunt/got {k:$(k)}
$execute positioned $(x) $(y) $(z) unless entity @a[distance=..48] run return 0
$execute positioned $(x) $(y) $(z) unless loaded ~ ~ ~ run return 0
$execute positioned $(x) $(y) $(z) if entity @e[type=minecraft:item,tag=arnis.hunt$(k),distance=..64] run return 0
$summon minecraft:item $(x) $(y) $(z) {Tags:["arnis.hunt","arnis.hunt$(k)","arnis.hunt_new"],Glowing:1b,Invulnerable:1b,PickupDelay:0,Age:-32768,Item:{id:"$(item)",count:1,components:{"minecraft:custom_name":{text:"$(label)",color:"gold",italic:false},"minecraft:custom_data":{arnis_hunt:1b,thing:$(k)}}}}
$execute store success score #ok arnis.story run spreadplayers $(x) $(z) 0 1 under $(sy) false @e[type=minecraft:item,tag=arnis.hunt_new]
$execute if score #ok arnis.story matches 0 run spreadplayers $(x) $(z) 0 3 false @e[type=minecraft:item,tag=arnis.hunt_new]
execute if score #duo arnis.story matches 1 as @e[type=minecraft:item,tag=arnis.hunt_new,tag=arnis.hunt0] run function arnis:story/hunt/heavy
tag @e[tag=arnis.hunt_new] remove arnis.hunt_new
