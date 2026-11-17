# Regrouping at the step's place: on once everyone playing is back, or once the wait
# after the first arrival runs out, so a friend who wandered off holds nobody up.
$execute positioned $(x) $(y) $(z) store result score #here arnis.story if entity @a[gamemode=!spectator,distance=..12]
execute store result score #all arnis.story if entity @a[gamemode=!spectator]
function arnis:story/bar
execute if score #here arnis.story matches 0 run return 0
$execute if score #gather arnis.story matches -1 positioned $(x) $(y) $(z) run function arnis:story/hunt/first
scoreboard players remove #gather arnis.story 1
execute if score #here arnis.story >= #all arnis.story run return run function arnis:story/hunt/reunion with storage arnis:story cur
execute if score #gather arnis.story matches ..0 run function arnis:story/hunt/reunion with storage arnis:story cur
