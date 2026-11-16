# The menu's choice, the walker each player last opened it on, and each walker's trait.
scoreboard objectives add arnis.npc trigger
scoreboard objectives add arnis.npc.t dummy
scoreboard objectives add arnis.npc.tr dummy
scoreboard players set #npc_t arnis.npc 0
scoreboard players set #2 arnis.npc 2
scoreboard players set #-1 arnis.npc -1
function arnis:npc/flags
function arnis:npc/lines
# Every menu button presses a trigger, and its "Triggered ..." line is noise in chat.
function arnis:npc/rule {rule:"sendCommandFeedback",value:"false"}
function arnis:npc/rule {rule:"send_command_feedback",value:"false"}
