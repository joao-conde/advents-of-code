import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day03"))(_.mkString).get
    val sacks = input.split("\n")

    val p1 = sacks
        .map(s => {
            val firstHalf = s.slice(0, s.length / 2)
            val secondHalf = s.slice(s.length / 2, s.length)
            val misplaced = firstHalf.find(c => secondHalf.contains(c)).get
            priority(misplaced)
        })
        .sum

    val p2 = (0 until sacks.length / 3)
        .map(i => {
            val (s1, s2, s3) = (sacks(i * 3), sacks(i * 3 + 1), sacks(i * 3 + 2))
            val badge = s1.find(c => s2.contains(c) && s3.contains(c)).get
            priority(badge)
        })
        .sum

    println("Part1: " + p1)
    println("Part2: " + p2)
}

def priority(item: Char): Int = item.toInt - (if (item.isUpper) 'A'.toInt - 27 else 'a'.toInt - 1)
