import scala.io.Source.fromFile
import scala.math.{abs, max, min}
import scala.util.Using

object Day15 {
    class Spot(val sx: Int, val sy: Int, val bx: Int, val by: Int) {
        val distance = abs(sx - bx) + abs(sy - by)
        val positiveSlopeXs = List(sx - sy - distance, sx - sy + distance)
        val negativeSlopeXs = List(sx + sy - distance, sx + sy + distance)
    }

    def main(args: Array[String]): Unit = {
        val input = Using(fromFile("input/day15"))(_.mkString).get

        val re = """Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)""".r
        val spots = input
            .split("\n")
            .map({ case re(sx, sy, bx, by) => Spot(sx.toInt, sy.toInt, bx.toInt, by.toInt) })

        val p1 = solveP1(spots)
        val p2 = solveP2(spots)
        println(s"Part1: $p1")
        println(s"Part2: $p2")
    }

    def solveP1(spots: Array[Spot]): Int = {
        val intervals = spots
            .map(spot => (spot, spot.distance - abs(spot.sy - 2000000)))
            .filter({ case (_, dx) => dx > 0 })
            .map({ case (spot, dx) => spot.sx - dx to spot.sx + dx })
        val minx = intervals.map(i => i.start).min
        val maxx = intervals.map(i => i.end).max
        (minx to maxx).count(x => intervals.exists(interval => interval.contains(x))) - 1
    }

    def solveP2(spots: Array[Spot]): BigInt = {
        val positives = spots.flatMap(_.positiveSlopeXs)
        val negatives = spots.flatMap(_.negativeSlopeXs)
        val posLine = findLine(positives)
        val negLine = findLine(negatives)
        val x = (posLine + negLine) / 2
        val y = (negLine - posLine) / 2
        BigInt(x) * 4000000 + y
    }

    def findLine(xs: Array[Int]): Int = (0 until xs.length)
        .flatMap(i => (i + 1 until xs.length).map(j => (i, j)))
        .map({ case (i, j) => (xs(i), xs(j)) })
        .find({ case (x0, x1) => abs(x0 - x1) == 2 })
        .map({ case (i, j) => min(i, j) + 1 })
        .get
}
