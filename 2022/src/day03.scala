import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day03"))(_.mkString).get
    val sacks = input.split("\n")
    val p1 = sacks
        .map(s => s.grouped(s.length / 2).toArray)
        .map({ case Array(x, y) => x.find(c => y.contains(c)).get })
        .map(priority)
        .sum
    val p2 = sacks
        .grouped(3)
        .map({ case Array(x, y, z) => x.find(c => y.contains(c) && z.contains(c)).get })
        .map(priority)
        .sum
    println(s"Part1: $p1")
    println(s"Part2: $p2")
}

def priority(item: Char): Int = item.toInt - (if (item.isUpper) 'A'.toInt - 27 else 'a'.toInt - 1)
