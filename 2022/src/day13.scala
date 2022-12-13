import scala.io.Source.fromFile
import scala.math.min
import scala.util.Using

type Token = Open | Close | Comma | Value
case class Open()
case class Close()
case class Comma()
case class Value(value: Int)

enum Comparison:
    case Greater, Equal, Lesser

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day13"))(_.mkString).get
    val pairs = input.split("\n\n").map(_.split("\n").map(parse))

    val p1 = pairs.zipWithIndex
        .map({ case (Array(p0, p1), i) => if (compare(p0, p1) == Comparison.Lesser) i + 1 else 0 })
        .sum

    val dividers = Array(parse("[[2]]"), parse("[[6]]"))
    val sorted = (pairs :+ dividers).flatten
        .sortWith((a, b) => compare(a, b) == Comparison.Lesser)
    val p2 = (sorted.indexWhere(_ == dividers(0)) + 1) * (sorted.indexWhere(_ == dividers(1)) + 1)

    println(s"Part1: ${p1}")
    println(s"Part2: ${p2}")
}

def parse(str: String): List[Any] = {
    var cur: List[Any] = List()
    var stack: List[List[Any]] = List()
    val tokens = tokenize(str)
    tokens.foreach({
        case Open() => {
            stack = stack :+ cur
            cur = List()
        }
        case Close() => {
            val prev = stack.last
            stack = stack.dropRight(1)
            cur = prev :+ cur
        }
        case Comma()      => {}
        case Value(value) => cur = cur :+ value
    })
    cur
}

def tokenize(str: String): List[Token] = {
    var tokens: List[Token] = List()
    var digits: List[Char] = List()
    for (c <- str) {
        c match {
            case '[' => tokens = tokens :+ Open()
            case ']' => {
                if (digits.nonEmpty) {
                    tokens = tokens :+ Value(digits.mkString.toInt)
                    digits = List()
                }
                tokens = tokens :+ Close()
            }
            case ',' => {
                if (digits.nonEmpty) {
                    tokens = tokens :+ Value(digits.mkString.toInt)
                    digits = List()
                }
                tokens = tokens :+ Comma()
            }
            case d => digits = digits :+ d
        }
    }
    tokens
}

def compare(left: Any, right: Any): Comparison = {
    (left, right) match {
        case (left: Int, right: Int) => {
            if (left == right)
                Comparison.Equal
            else if (left < right)
                Comparison.Lesser
            else
                Comparison.Greater
        }
        case (left: List[Any], right: List[Any]) => {
            val len = min(left.length, right.length)
            val comparisons = (0 until len)
                .map(i => compare(left(i), right(i)))
                .dropWhile(c => c == Comparison.Equal)
            val comparison = if (comparisons.isEmpty) Comparison.Equal else comparisons(0)

            (comparison, left.length, right.length) match {
                case (Comparison.Equal, x, y) if x < y  => Comparison.Lesser
                case (Comparison.Equal, x, y) if x > y  => Comparison.Greater
                case (Comparison.Equal, x, y) if x == y => Comparison.Equal
                case _                                  => comparison
            }
        }
        case (left: Int, right: List[Any]) => compare(List(left), right)
        case (left: List[Any], right: Int) => compare(left, List(right))
    }
}
