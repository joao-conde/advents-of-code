import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day03"))(_.mkString).get
    val sacks = input.split("\n")
    val p1 = sacks
        .map(s => (s.slice(0, s.length / 2), s.slice(s.length / 2, s.length)))
        .map({ case (x, y) => x.find(c => y.contains(c)).get })
        .map(priority)
        .sum
    val p2 = (0 until sacks.length / 3)
        .map(i => sacks.slice(i * 3, i * 3 + 3))
        .map({ case Array(x, y, z) => x.find(c => y.contains(c) && z.contains(c)).get })
        .map(priority)
        .sum
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def priority(item: Char): Int = item.toInt - (if (item.isUpper) 'A'.toInt - 27 else 'a'.toInt - 1)
