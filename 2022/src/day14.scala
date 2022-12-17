import scala.collection.mutable.Set
import scala.io.Source.fromFile
import scala.math.{abs, max}
import scala.util.Using

type Point = (Int, Int)
type OccupiedFn = (occupied: Set[Point], bottom: Int, point: Point) => Boolean

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day14"))(_.mkString).get
    val rockLines = input
        .split("\n")
        .map(
          _.split("->").map(_.split(",").map(_.trim.toInt)).map({ case Array(x, y) => (x, y) })
        )

    val occupied1: OccupiedFn = (occupied, bottom, point) => occupied.contains(point)
    val occupied2: OccupiedFn = (occupied, bottom, point) =>
        occupied.contains(point) || point(1) == bottom

    val p1 = sandUnits(computeRocks(rockLines), occupied1)
    val p2 = sandUnits(computeRocks(rockLines), occupied2) + 1
    println(s"Part1: $p1")
    println(s"Part2: $p2")
}

def computeRocks(rockLines: Array[Array[Point]]): Set[Point] = {
    val rocks: Set[Point] = Set()
    for (path <- rockLines) {
        var cur = path(0)
        for (point <- path.drop(1)) {
            rocks.addAll(rockPath(cur, point))
            cur = point
        }
    }
    rocks
}

def rockPath(p0: Point, p1: Point): List[Point] = {
    val ((x0, y0), (x1, y1)) = (p0, p1)
    val delta = ((x1 - x0), (y1 - y0))
    val len = max(abs(delta(0)), abs(delta(1)))
    (1 to len).scanLeft(p0)((rock, _) => (rock(0) + delta(0).sign, rock(1) + delta(1).sign)).toList
}

def sandUnits(occupied: Set[Point], occupiedFn: OccupiedFn, start: Point = (500, 0)): Int = {
    val bottom = occupied.map((_, y) => y).max + 2
    LazyList
        .from(1)
        .takeWhile(_ => dropSand(occupied, occupiedFn, bottom, start).getOrElse(start) != start)
        .toList
        .last
}

def dropSand(
    occupied: Set[Point],
    occupiedFn: OccupiedFn,
    bottom: Int,
    start: Point
): Option[Point] = {
    var cur = start
    while (cur(1) <= bottom) {
        val below = (cur(0), cur(1) + 1)
        val downLeft = (cur(0) - 1, cur(1) + 1)
        val downRight = (cur(0) + 1, cur(1) + 1)

        if (!occupiedFn(occupied, bottom, below)) cur = below
        else if (!occupiedFn(occupied, bottom, downLeft)) cur = downLeft
        else if (!occupiedFn(occupied, bottom, downRight)) cur = downRight
        else {
            occupied.add(cur)
            return Some(cur)
        }
    }
    None
}
