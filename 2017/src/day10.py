def knot_hash(raw_lenghts):
    ascii_lenghts = [ord(x) for x in [c for c in raw_lenghts]]
    ascii_lenghts = ascii_lenghts + [17, 31, 73, 47, 23]

    pos = skip = 0
    sparse_hash = [x for x in range(256)]
    for _ in range(64):
        sparse_hash, pos, skip = knot_hash_logic(sparse_hash, ascii_lenghts, pos, skip)

    dense_hash = []
    for block in [sparse_hash[i * 16 : (i * 16) + 16] for i in range(16)]:
        xor = 0
        for n in block:
            xor ^= n
        dense_hash.append(xor)

    return "".join(f"{n:02x}" for n in dense_hash)


def knot_hash_logic(l, lengths, pos=0, skip=0):
    for length in lengths:
        l = reverse_portion_circular_list(l, pos, pos + length)
        pos = (pos + length + skip) % len(l)
        skip += 1
    return l, pos, skip


def reverse_portion_circular_list(l, start, end):
    from_pos = [x % len(l) for x in range(start, end)]
    to_pos = [x % len(l) for x in range(start, end)][::-1]
    num_swaps = len(from_pos) // 2
    for i in range(num_swaps):
        if from_pos[i] == to_pos[i]:
            return l
        else:
            tmp = l[from_pos[i]]
            l[from_pos[i]] = l[to_pos[i]]
            l[to_pos[i]] = tmp
    return l


if __name__ == "__main__":
    input_file = open("input/day10")
    raw_lenghts = input_file.read()
    input_file.close()

    # PART 1
    lengths = [int(length) for length in raw_lenghts.split(",")]
    l, _, _ = knot_hash_logic([x for x in range(256)], lengths)
    print(f"Result of multiplying the first two numbers in the list: {l[0] * l[1]}")

    # PART 2
    print(f"Knot hash: {knot_hash(raw_lenghts)}")
