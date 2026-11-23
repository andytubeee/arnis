"""Checks a City Stories data file against assets/stories/FORMAT.md. Usage: python3 check_stories.py FILE..."""
import json, sys, re
ROLES={"park","food_shop","school","viewpoint","faith","bridge","water","station","civic","culture","landmark","any"}
FINALES={"gift","lost_dog","time_capsule","surprise_party","band_reunion","riddle_treasure","friendly_ghost","stargazer","cat_mayor","message_bottle","allay_courier","founders_festival","secret_garden","golems_heart","picnic_bakeoff","postcard_tour"}
item=re.compile(r"^minecraft:[a-z0-9_]+$")
bad=0; keys=set()
def err(k,m):
    global bad; bad+=1; print(f"{k}: {m}")
for f in sys.argv[1:]:
    for s in json.load(open(f))["stories"]:
        k=s.get("key","?")
        if k in keys: err(k,"duplicate key")
        keys.add(k)
        if not re.match(r"^[a-z][a-z0-9_]*$",k): err(k,"key not snake_case")
        if not s.get("title"): err(k,"no title")
        if ("series" in s)!=("episode" in s): err(k,"series and episode go together")
        if s.get("finale") not in FINALES: err(k,f"finale {s.get('finale')}")
        if s.get("finale")=="gift" and not item.match(s.get("gift","")): err(k,"gift needs an item id")
        st=s.get("steps",[])
        if not 3<=len(st)<=6: err(k,f"{len(st)} steps")
        riddles=0
        for i,p in enumerate(st):
            kind=p.get("kind"); last=i==len(st)-1
            r=p.get("roles",[])
            if not r or r[-1]!="any" or any(x not in ROLES for x in r): err(k,f"step {i} roles {r}")
            need={"talk":["npc","line"],"riddle":["hint","line"],"search":["thing","item","line"],"choice":["npc","line","options","answer","wrong","right"]}.get(kind)
            if need is None: err(k,f"step {i} kind {kind}"); continue
            for n in need:
                if n not in p: err(k,f"step {i} {kind} lacks {n}")
            if kind=="riddle":
                riddles+=1
                if i==0: err(k,"a riddle can't be the first step")
                if len(p.get("hint",""))>40: err(k,f"step {i} hint over 40")
            if kind=="search":
                if not item.match(p.get("item","")): err(k,f"step {i} item")
                if any(c in p.get("thing","") for c in '"\\'): err(k,f"step {i} thing has quote")
            if kind=="choice":
                o=p.get("options",[])
                if len(o)!=3 or p.get("answer") not in (0,1,2): err(k,f"step {i} options/answer")
            if last and kind!="talk": err(k,"last step must be talk")
            out=p.get("right") if kind=="choice" else p.get("line","")
            for t in [p.get("line",""),p.get("right",""),p.get("wrong",""),p.get("hint","")]+p.get("options",[]):
                if len(t)>200: err(k,f"step {i} text over 200")
            if last:
                if "{next}" in (out or ""): err(k,"ending uses {next}")
            else:
                nxt=st[i+1].get("kind")
                if nxt=="riddle" and "{next}" in (out or ""): err(k,f"step {i} names the riddle's place with {{next}}")
                if nxt!="riddle" and "{next}" not in (out or ""): err(k,f"step {i} lacks {{next}}")
        if riddles>2: err(k,"more than 2 riddles")
        h=s.get("hunt")
        if h:
            if len(st)<3 or st[-2].get("kind")!="talk": err(k,"hunt needs a talk second-to-last step")
            if "{items}" not in h.get("line",""): err(k,"hunt line lacks {items}")
            its=h.get("items",[])
            if len(its)!=3 or any(len(x)!=2 or not item.match(x[1]) or any(c in x[0] for c in '"\\') for x in its): err(k,"hunt items")
print(f"{len(keys)} stories, {bad} problems"); sys.exit(1 if bad else 0)
