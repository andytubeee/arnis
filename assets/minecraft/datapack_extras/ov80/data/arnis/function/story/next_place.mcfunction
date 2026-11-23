# Points the menu's directions at the current stop: 1 while the story runs, 0 after.
execute if score #step arnis.story >= #steps arnis.story run return 0
function arnis:story/current
# Nobody gives a riddle's answer away.
execute if data storage arnis:story cur{kind:"riddle"} run return 0
data modify storage arnis:npc where set from storage arnis:story cur
# On the hunt, the way to this player's own thing.
execute if score #step arnis.story = #hunt_step arnis.story if score #hunt arnis.story matches 1 if score @s arnis.story.job matches 0.. run function arnis:story/hunt/mine
return 1
