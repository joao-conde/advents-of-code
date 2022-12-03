import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day03"))(_.mkString).get
    val sacks = input.split("\n")

    val p1 = sacks
        .map(s => {
            val firstHalf = s.slice(0, s.length / 2)
            val secondHalf = s.slice(s.length / 2, s.length)
            val items = charSet(firstHalf)
            val misplaced = secondHalf.find(items.contains).get
            priority(misplaced)
        })
        .sum

    val p2 = (0 until sacks.length / 3)
        .map(i => {
            val s1 = sacks(i * 3)
            val s2 = sacks(i * 3 + 1)
            val s3 = sacks(i * 3 + 2)
            val x = charSet(s1).intersect(charSet(s2))
            val dup = s3.find(x.contains).get
            priority(dup)
        })
        .sum

    println("Part1: " + p1)
    println("Part2: " + p2)
}

def charSet(str: String): Set[Char] = str.foldLeft(Set[Char]())((acc, c) => acc + c)

def priority(item: Char): Int = item.toInt - (if (item.isUpper) 'A'.toInt - 27 else 'a'.toInt - 1)
