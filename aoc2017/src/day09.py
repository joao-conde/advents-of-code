input_file = open("input/day09")
groups_str = input_file.read()
input_file.close()

ESCAPE_CHAR, GARBAGE_OPEN, GARBAGE_CLOSE, GROUP_OPEN, GROUP_CLOSE = (
    "!",
    "<",
    ">",
    "{",
    "}",
)

while ESCAPE_CHAR in groups_str:
    escape_char_idx = groups_str.index(ESCAPE_CHAR)
    groups_str = groups_str[:escape_char_idx] + groups_str[escape_char_idx + 2 :]

removed_garbage = 0
while GARBAGE_OPEN in groups_str:  # [)
    open_tag_idx = groups_str.index(GARBAGE_OPEN)
    close_tag_idx = groups_str.index(GARBAGE_CLOSE)
    groups_str = groups_str[:open_tag_idx] + groups_str[close_tag_idx + 1 :]
    removed_garbage += close_tag_idx - open_tag_idx - 1

nestedness_lvl, score = 0, 0
for c in groups_str:
    if c == GROUP_OPEN:
        nestedness_lvl += 1
    elif c == GROUP_CLOSE:
        score += nestedness_lvl
        nestedness_lvl -= 1

print(f"Total score for all groups: {score}")
print(f"Total garbage removed: {removed_garbage}")
