#Link to problem: https://adventofcode.com/2017/day/20

import re, math

def get_particle_properties(particle_id, particle_info):
    pattern = "p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>"
    [px, py, pz, vx, vy, vz, ax, ay, az] = [int(x) for x in re.findall(pattern, particle_info)[0]]
    return {
        "pid": particle_id,
        "px": px,
        "py": py,
        "pz": pz,
        "vx": vx, 
        "vy": vy,
        "vz": vz, 
        "ax": ax,
        "ay": ay, 
        "az": az
    }

src = "../res/d20"
input_file = open(src)
particle_input = input_file.read().split('\n')
input_file.close()

# PART 1
particles_props = [get_particle_properties(pid, particle_info) for (pid, particle_info) in enumerate(particle_input)]
particles_abs_accel = [(math.sqrt(info["ax"]**2 + info["ay"]**2 + info["az"]**2), info["pid"]) for info in particles_props]
particles_abs_accel.sort() # sorts by acceleration magnitude
print(f'(Part1) Closest particle to <0,0,0> in the long term: {particles_abs_accel[0][1]}')
