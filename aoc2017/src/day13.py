#Link to problem: https://adventofcode.com/2017/day/13


class Layer:

    def __init__(self, range):
        self.range = range
        self.scan_area = ['E'] * range
        self.scan_dir = 1 #default, 1 to go "down", -1 to go "up"

        if range > 0: self.scan_area[0] = 'S' #scanner initial position


    def top(self):

        if len(self.scan_area) == 0:
            return 'E'
        else:
            return self.scan_area[0]

    def advance_scanner(self):
        
        if self.range > 0:
            scanner_idx = self.scan_area.index('S')
            self.scan_area[scanner_idx] = 'E'
            scanner_idx += self.scan_dir
            self.scan_area[scanner_idx] = 'S'

            if scanner_idx == self.range-1: self.scan_dir = -1
            if scanner_idx == 0: self.scan_dir = 1



    def __str__(self):
        info = "LAYER RANGE: " + str(self.range) + "\n"

        for el in self.scan_area:
            info += el + " "


        return info


#For current repos config path is '../res/d13.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")


src = '../res/d13.txt'


input_file = open(src)
layers = input_file.read()
layers = [layer.replace(':', '').split() for layer in layers.split('\n')]
input_file.close()

#PART 1

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



#Compute severity of getting caught
severity = 0
picoseconds = len(firewall)
for ps in range(picoseconds):

    if firewall[ps].top() == 'S':
        severity += ps * firewall[ps].range #depth * range

    for layer in firewall: layer.advance_scanner()


print("\nSeverity of the whole trip:", severity)


