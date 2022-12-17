import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day04"))(_.mkString).get
    val pairs = input.split("\n").map(_.split(",").flatMap(_.split("-").map(_.toInt)))
    val ranges = pairs.map({ case Array(x, y, z, w) => (x to y, z to w) })
    val p1 = ranges.count((r1, r2) => r1.forall(r2.contains) || r2.forall(r1.contains))
    val p2 = ranges.count((r1, r2) => r1.exists(r2.contains))
    println(s"Part1: $p1")
    println(s"Part2: $p2")
}
