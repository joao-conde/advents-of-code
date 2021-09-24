input_file = open("input/day13")
layers = input_file.read().split("\n")
input_file.close()

layers = [layer.replace(":", "").split() for layer in layers]
layers_dict = {}
for pair in layers:
    layers_dict[int(pair[0])] = int(pair[1])

# PART 1
severity, max_ps = 0, max([x for x in layers_dict])
for ps in range(max_ps + 1):
    if ps in layers_dict:
        if (
            ps % (2 * (layers_dict[ps] - 1)) == 0
        ):  # scanner is at position 0 at frequency 2 * (range - 1)
            severity += ps * layers_dict[ps]

print("Severity of the whole trip:", severity)

# PART 2
min_delay = 0
while 0 in [
    (ps + min_delay) % (2 * (layers_dict[ps] - 1))
    for ps in range(max_ps + 1)
    if ps in layers_dict
]:
    min_delay += 1
print("Fewest packet delay to pass through the firewall:", min_delay)
