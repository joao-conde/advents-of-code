import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day02"))(_.mkString).get
    val rounds = input.split("\n").map(_.split(" "))

    val alias = Map("X" -> "A", "Y" -> "B", "Z" -> "C")
    val weakness = Map("A" -> "B", "B" -> "C", "C" -> "A")
    val points = Map("A" -> 1, "B" -> 2, "C" -> 3)

    def score(x: String, y: String): Int =
        points(y) + (if (x == y) 3 else if (weakness(y) != x) 6 else 0)

    def shape(x: String, y: String): String =
        if (y == "X") weakness.find(_._2 == x).map(_._1).get else if (y == "Y") x else weakness(x)

    val p1 = rounds.map({ case Array(x, y) => score(x, alias(y)) }).sum
    val p2 = rounds.map({ case Array(x, y) => score(x, shape(x, y)) }).sum
    println("Part1: " + p1)
    println("Part2: " + p2)
}
