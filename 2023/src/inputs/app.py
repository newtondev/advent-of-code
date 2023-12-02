from collections import defaultdict
ll = [x for x in open('02.txt').read().strip().split('\n')]

p1 = 0
p2 = 0
for l in ll:
	gameid = int(l.split(":")[0].split(" ")[1])
	l = l.split(":")[1]
	possible = True
	mincnts = defaultdict(int)
	for s in l.split(";"):
		cnts = defaultdict(int)
		for rev in s.split(", "):
			rev = rev.strip()
			cnts[rev.split(" ")[1]]+=int(rev.split(" ")[0])
		for k,v in cnts.items():
			mincnts[k] = max(mincnts[k], v)
		if not (cnts["red"] <= 12 and cnts["green"] <= 13 and cnts["blue"] <= 14):
			possible=False
	if possible:
		p1 += gameid
	p2 += mincnts["red"]*mincnts["green"]*mincnts["blue"]
print(p1)
print(p2)
