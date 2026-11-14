# Points the menu's directions at the current stop: 1 while the story runs, 0 after.
execute if score #step arnis.story >= #steps arnis.story run return 0
function arnis:story/current
data modify storage arnis:npc where set from storage arnis:story cur
return 1
