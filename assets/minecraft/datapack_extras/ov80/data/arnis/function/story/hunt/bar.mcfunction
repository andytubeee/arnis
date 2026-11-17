# A job's bar, with the way from one of the players on it.
$function arnis:story/hunt/where {k:$(k)}
function arnis:npc/hint
$bossbar set arnis:hunt$(k) name [{"text":"Your job: the ","color":"gold"},{"storage":"arnis:story","nbt":"hunt[$(k)].label","color":"yellow"},{"text":" at ","color":"gold"},{"storage":"arnis:story","nbt":"hunt[$(k)].name","color":"yellow"},{"text":", ","color":"gold"},{"score":{"name":"#m","objective":"arnis.npc"},"color":"white"},{"text":" m ","color":"white"},{"storage":"arnis:npc","nbt":"way","color":"white"}]
