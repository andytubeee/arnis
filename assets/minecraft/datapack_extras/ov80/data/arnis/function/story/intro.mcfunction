scoreboard players set @s arnis.story.seen 0
function arnis:story/current
tellraw @s [{"text":"City Stories: ","color":"gold","bold":true},{"text":"someone at ","color":"yellow","bold":false},{"storage":"arnis:story","nbt":"cur.name","color":"white","bold":false},{"text":" is waiting for you. Look for the glowing person and right-click them. The bar at the top shows where to go next, and the pause menu points the way. Anyone can do a step for everyone.","color":"yellow","bold":false}]
