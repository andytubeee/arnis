# The step everyone is on, how many each player has been granted, and the pause page's choice.
scoreboard objectives add arnis.story dummy
scoreboard objectives add arnis.story.seen dummy
scoreboard objectives add arnis.story.ctl trigger
# Which of the hunt's things each player is after.
scoreboard objectives add arnis.story.job dummy
execute unless score #hunt arnis.story matches 0.. run scoreboard players set #hunt arnis.story 0
bossbar add arnis:hunt0 {"text":"Hunt"}
bossbar add arnis:hunt1 {"text":"Hunt"}
bossbar add arnis:hunt2 {"text":"Hunt"}
bossbar set arnis:hunt0 color green
bossbar set arnis:hunt1 color green
bossbar set arnis:hunt2 color green
scoreboard players set #story_t arnis.story 0
function arnis:story/plan
# Which story of the run is playing, and its step; a new world starts at the first.
execute unless score #story arnis.story matches 0.. run scoreboard players set #story arnis.story 0
execute unless score #step arnis.story matches 0.. run scoreboard players set #step arnis.story 0
execute if score #story arnis.story >= #stories arnis.story run scoreboard players set #story arnis.story 0
function arnis:story/open
execute unless score #errand arnis.story matches 0.. run function arnis:story/errand/reset
bossbar add arnis:story {"text":"City Stories"}
bossbar set arnis:story color yellow
function arnis:story/bar
