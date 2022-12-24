import scala.collection.mutable.Set
import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day24"))(_.mkString).get

    val maxi = input.split("\n").length - 1
    val maxj = input.split("\n")(0).length - 1

    val directions = List('^', 'v', '<', '>')
    val blizzards = input
        .split("\n")
        .zipWithIndex
        .flatMap((r, i) =>
            r.zipWithIndex.filter((c, j) => directions.contains(c)).map((c, j) => (c, i, j))
        )
        .toList

    val p1 = bfs(blizzards, maxi, maxj)
    println(p1)
}

def bfs(blizzards: List[(Char, Int, Int)], maxi: Int, maxj: Int): Int = {
    val moves = List((0, 0), (-1, 0), (1, 0), (0, -1), (0, 1))

    val visited: Set[(Int, Int, Int)] = Set()

    var queue = List((0, 1, blizzards, 0))
    while (queue.nonEmpty) {
        var (i, j, bzs, time) = queue.head
        queue = queue.tail

        if (i == maxi && j == maxj - 1) {
            println(s"FOUND IT $time")
            return time
        }

        if (!visited.contains((i, j, time))) {
            val nextBlizzards = step(bzs, 0, maxi, 0, maxj)
            val nextBlizzardsPositions = nextBlizzards.map((_, i, j) => (i, j)).toSet

            val next = moves
                .map((di, dj) => (i + di, j + dj, nextBlizzards, time + 1))
                .filter((i, j, _, _) =>
                    i > 0 && j > 0 && ((i < maxi && j < maxj) || (i == maxi && j == maxj - 1))
                )
                .filter((i, j, _, _) => !nextBlizzardsPositions.contains((i, j)))

            queue = queue ::: next
        }
        visited.addOne((i, j, time))
    }

    0
}

def step(
    blizzards: List[(Char, Int, Int)],
    mini: Int,
    maxi: Int,
    minj: Int,
    maxj: Int
): List[(Char, Int, Int)] = {
    blizzards
        .map({
            case ('^', i, j) => ('^', i - 1, j)
            case ('v', i, j) => ('v', i + 1, j)
            case ('<', i, j) => ('<', i, j - 1)
            case ('>', i, j) => ('>', i, j + 1)
        })
        .map((c, i, j) => (c, warp(i, mini, maxi), warp(j, minj, maxj)))
}

def warp(x: Int, minx: Int, maxx: Int): Int =
    if (x == minx) maxx - 1 else if (x == maxx) minx + 1 else x
