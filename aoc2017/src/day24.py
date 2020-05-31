#Link to problem: https://adventofcode.com/2017/day/24

def findBridges(components, bridge, bridges):
  possibleNext = [c for c in components if bridge[-1][1] in c and c not in bridge and [c[1], c[0]] not in bridge]
  if len(possibleNext) != 0:
    for c in possibleNext:
      cBridge = bridge.copy()
      if cBridge[-1][1] == c[0]:
        cBridge.append(c)
      elif cBridge[-1][1] == c[1]:
        cBridge.append([c[1], c[0]])
      findBridges(components, cBridge, bridges)
  else:
    bridges.append(bridge)
  return bridges

src = "../res/d24"
inputFile = open(src)
components = inputFile.read().split('\n')
components = [[int(pin) for pin in component.split("/")] for component in components]
inputFile.close()

# PART 1
startComponents = [c for c in components if 0 in c]
otherComponents = [c for c in components if 0 not in c]

bridges = []
for c in startComponents: bridges.extend(findBridges(otherComponents, [c], []))
bridgesSums = [sum([sum(c) for c in b]) for b in bridges]
print(f'(Part1) Strongest bridge strength: {max(bridgesSums)}')

# PART 2
maxLenBridge = max([len(b) for b in bridges])
longestBridges = [b for b in bridges if len(b) >= maxLenBridge]
longestBridgesSums = [sum([sum(c) for c in b]) for b in longestBridges]
print(f'(Part2) Longest and strongest bridge strength *wink wink*: {max(longestBridgesSums)}')
