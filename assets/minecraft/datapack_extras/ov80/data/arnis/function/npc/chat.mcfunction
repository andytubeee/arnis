# Small talk; with a story on, one time in four they mention one of its places.
execute store result storage arnis:npc pick.i int 1 run random value 0..19
execute store result score #r arnis.npc run random value 0..3
execute if score #stories arnis.npc matches 1 if score #r arnis.npc matches 0 run return run function arnis:npc/gossip
data modify storage arnis:npc pick.list set value "chat"
function arnis:npc/say with storage arnis:npc pick
