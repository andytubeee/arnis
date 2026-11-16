execute store result storage arnis:npc pick.i int 1 run random value 0..9
data modify storage arnis:npc pick.list set value "sorry"
function arnis:npc/say with storage arnis:npc pick
