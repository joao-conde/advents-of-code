import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day03"))(_.mkString).get
    val sacks = input.split("\n")
    val p1 = sacks
        .map(s => s.sliding(s.length / 2, s.length / 2).toList)
        .map(halves => halves(0).find(c => halves(1).contains(c)).get)
        .map(priority)
        .sum
    val p2 = sacks
        .sliding(3, 3)
        .map({ case Array(x, y, z) => x.find(c => y.contains(c) && z.contains(c)).get })
        .map(priority)
        .sum
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def priority(item: Char): Int = item.toInt - (if (item.isUpper) 'A'.toInt - 27 else 'a'.toInt - 1)
