#Link to problem: https://adventofcode.com/2017/day/20

import re, math
from functools import reduce

def get_particle_properties(particle_id: int, particle_info: str):
    pattern = "p=<(.*),(.*),(.*)>, v=<(.*),(.*),(.*)>, a=<(.*),(.*),(.*)>"
    [px, py, pz, vx, vy, vz, ax, ay, az] = [int(x) for x in re.findall(pattern, particle_info)[0]]
    return particle_id, {
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
particles_abs_accel = [(math.sqrt(info["ax"]**2 + info["ay"]**2 + info["az"]**2), pid) for pid, info in particles_props]
particles_abs_accel.sort() # sorts by acceleration magnitude
print(f'(Part1) Closest particle to <0,0,0> in the long term: {particles_abs_accel[0][1]}')

# PART 2 (gave up on quadratic solver based solution to find if two particles ever collide)
SIMULATION_STEPS = 500
axes = ["x", "y", "z"]
position_keys = ["px", "py", "pz"]
for _ in range(SIMULATION_STEPS):
    for _, p in particles_props:
        for axis in axes:
            p["v" + axis] += p["a" + axis]
            p["p" + axis] += p["v" + axis]

    collided = []
    for pid1, p1 in particles_props:
        for pid2, p2 in particles_props:
            if pid1 == pid2: continue
            if reduce(lambda a, b: a and b, [p1[k] == p2[k] for k in position_keys]):
                collided.extend([pid1, pid2])

    particles_props = [(pid, _) for (pid, _) in particles_props if pid not in collided]

print(f'(Part2) Particles left after all collisions are resolved: {len(particles_props)}')
