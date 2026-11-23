# A short breather after a story, then the next one, round and round the run.
scoreboard players remove #between arnis.story 1
execute if score #between arnis.story matches 1.. run return 0
scoreboard players add #story arnis.story 1
execute if score #story arnis.story >= #stories arnis.story run scoreboard players set #story arnis.story 0
scoreboard players set #step arnis.story 0
scoreboard players set #beat_t arnis.story 0
function arnis:story/open
function arnis:story/begin
