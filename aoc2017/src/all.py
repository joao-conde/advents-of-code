import os

print("🎄 Advent of Code 2020 🐍\n")
for day in range(1, 26):
    print(f"> Day {day}")
    day = day if day >= 10 else f"0{day}"
    os.system("python src/day%s.py" % day)
    print()
