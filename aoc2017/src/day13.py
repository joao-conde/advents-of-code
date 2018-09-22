#Link to problem: https://adventofcode.com/2017/day/13


class Layer:

    def __init__(self, range):
        self.range = range
        self.scan_area = ['E'] * range

    def top(self):

        if len(self.scan_area) == 0:
            return 'E'
        else:
            return self.scan_area[0]

    def advance_scanner(self):
        print("SCANNER ADVANCE")

    def __str__(self):
        return "LAYER RANGE: " + str(self.range)


#For current repos config path is '../res/d13.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")

#TODO: advance SCANNERS, remove this hardcoded input, place scanners at begin position

src = '../res/d13.txt'



input_file = open(src)
layers = input_file.read()
layers = [layer.replace(':', '').split() for layer in layers.split('\n')]


input_file.close()


#Build firewall
firewall = []
depth = 0
while len(layers) > 0:

    if depth == int(layers[0][0]):
        layer = Layer(int(layers[0][1]))
        layers.remove(layers[0])
    else:
        layer = Layer(0)

    firewall.append(layer)
    depth += 1



#Compute severity of gettingcaught
severity = 0
picoseconds = len(firewall)
for ps in range(picoseconds):

    if firewall[ps].top() == 'S':
        severity += ps * firewall[ps].range #depth * range


print("Severity:", severity)


