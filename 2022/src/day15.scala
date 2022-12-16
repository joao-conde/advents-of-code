import scala.io.Source.fromFile
import scala.math.{abs, max, min}
import scala.util.Using

type Point = (Int, Int)

type Equation = (p: Point) => Boolean;

class Area(sx: Int, sy: Int, bx: Int, by: Int) {
    val d = manhattan((sx, sy), (bx, by))
    val m = 1

    val inTopLeft: Equation = p => p(1) >= m * p(0) + getB(sx, sy, m) - d
    val inTopRight: Equation = p => p(1) >= -m * p(0) + getB(sx, sy, -m) - d
    val inBotLeft: Equation = p => p(1) <= -m * p(0) + getB(sx, sy, -m) + d
    val inBotRight: Equation = p => p(1) <= m * p(0) + getB(sx, sy, m) + d

    def contains(p: Point): Boolean =
        List(inTopLeft, inTopRight, inBotLeft, inBotRight).forall(l => l(p))

    def manhattan(p0: Point, p1: Point): Int = abs(p0(0) - p1(0)) + abs(p0(1) - p1(1))

    def getB(sx: Int, sy: Int, m: Int): Int = sy - sx * m
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day15"))(_.mkString).get

    val re = """Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)""".r

    val areas = input
        .split("\n")
        .map({ case re(sx, sy, bx, by) =>
            Area(sx.toInt, sy.toInt, bx.toInt, by.toInt)
        })
        .toList

    val p1 = (-10000000 to 10000000)
        .map(x => (x, 2000000))
        .count(p => areas.exists(area => area.contains(p))) - 1

    println(s"Part1: ${p1}")
}
