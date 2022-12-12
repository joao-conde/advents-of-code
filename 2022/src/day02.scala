import scala.io.Source.fromFile
import scala.util.Using

val alias = Map("X" -> "A", "Y" -> "B", "Z" -> "C")
val weakness = Map("A" -> "B", "B" -> "C", "C" -> "A")
val points = Map("A" -> 1, "B" -> 2, "C" -> 3)

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day02"))(_.mkString).get
    val rounds = input.split("\n").map(_.split(" "))
    val p1 = rounds.map({ case Array(op, me) => scoreMe(alias(me), op) }).sum
    val p2 = rounds.map({ case Array(op, strat) => scoreMe(move(op, strat), op) }).sum
    println(s"Part1: $p1")
    println(s"Part2: $p2")
}

def scoreMe(me: String, op: String): Int =
    points(me) + (if (op == me) 3 else if (weakness(me) != op) 6 else 0)

def move(op: String, strategy: String): String =
    strategy match {
        case "X" => weakness.find(_._2 == op).map(_._1).get
        case "Y" => op
        case _   => weakness(op)
    }
