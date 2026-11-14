# Grants this player every step done so far, one a call, so late joiners see the story too.
execute if score @s arnis.story.seen >= #step arnis.story run return 0
execute store result storage arnis:story g.k int 1 run scoreboard players get @s arnis.story.seen
function arnis:story/grant with storage arnis:story g
scoreboard players add @s arnis.story.seen 1
function arnis:story/catch_up
