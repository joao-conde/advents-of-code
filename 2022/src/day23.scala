import scala.io.Source.fromFile
import scala.util.Using

object Day23 {
    def main(args: Array[String]): Unit = {
        val input = Using(fromFile("input/day23"))(_.mkString).get
        val elves = input
            .split("\n")
            .zipWithIndex
            .flatMap((r, i) => r.zipWithIndex.filter((c, j) => c == '#').map((c, j) => (i, j)))
            .toSet

        val region = (0 until 10).foldLeft(elves)((state, r) => round(state, r))
        val minx = region.map((x, _) => x).min
        val maxx = region.map((x, _) => x).max
        val miny = region.map((_, y) => y).min
        val maxy = region.map((_, y) => y).max
        val area = (1 + maxx - minx) * (1 + maxy - miny)
        val p1 = area - region.size

        var (p2, changed, state) = (0, true, elves)
        while (changed) {
            val nextState = round(state, p2)
            p2 += 1
            changed = state != nextState
            state = nextState
        }

        println(s"Part1: $p1")
        println(s"Part2: $p2")
    }

    def round(elves: Set[(Int, Int)], first: Int): Set[(Int, Int)] = {
        val directions = List(
          List((-1, 0), (-1, 1), (-1, -1)),
          List((1, 0), (1, 1), (1, -1)),
          List((0, -1), (-1, -1), (1, -1)),
          List((0, 1), (-1, 1), (1, 1))
        )

        val propositions = elves.toList
            .map(proposition(directions, elves, _, first))
            .foldLeft(Map[(Int, Int), Int]().withDefaultValue(0))((ps, p) =>
                ps.updated(p, ps(p) + 1)
            )

        elves.foldLeft(Set[(Int, Int)]())((next, elf) => {
            val p = proposition(directions, elves, elf, first)
            next + (if (propositions(p) == 1) p else elf)
        })
    }

    def proposition(
        directions: List[List[(Int, Int)]],
        elves: Set[(Int, Int)],
        elf: (Int, Int),
        first: Int
    ): (Int, Int) = {
        if (isolated(directions, elves, elf))
            return elf

        val (dx, dy) = (0 until directions.length)
            .map(i => (i + first) % directions.length)
            .map(directions(_))
            .find(canMove(_, elves, elf))
            .getOrElse(List((0, 0)))
            .head
        (elf(0) + dx, elf(1) + dy)
    }

    def isolated(
        directions: List[List[(Int, Int)]],
        elves: Set[(Int, Int)],
        elf: (Int, Int)
    ): Boolean =
        directions.forall(direction =>
            direction.map((di, dj) => (elf(0) + di, elf(1) + dj)).forall(!elves.contains(_))
        )

    def canMove(direction: List[(Int, Int)], elves: Set[(Int, Int)], elf: (Int, Int)): Boolean =
        direction.map((di, dj) => (elf(0) + di, elf(1) + dj)).forall(!elves.contains(_))
}
