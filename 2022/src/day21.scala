import scala.io.Source.fromFile
import scala.util.Using

type Expression = (String, String, String) | BigDecimal

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day21"))(_.mkString).get

    val re1 = """(\w+): (-?\d+)""".r
    val re2 = """(\w+): (\w+) ([+\-*\/]) (\w+)""".r
    val expressions = input
        .split("\n")
        .foldLeft(Map[String, Expression]())((acc, x) => {
            val expression = x match {
                case re1(m1, num)        => m1 -> BigDecimal(num)
                case re2(m1, m2, op, m3) => m1 -> (m2, op, m3)
            }
            acc + expression
        })

    val p1 = evaluate("root", expressions)

    // solution is interception of two lines since the equation is linear
    // m1 * x + b1 = m2 * x + b2 <=> x = (b1 - b2) / (m2 - m1)
    val (monkey1, _, monkey2) = expressions("root"): @unchecked
    val (slope1, b1) = line(monkey1, expressions)
    val (slope2, b2) = line(monkey2, expressions)
    val p2 = (b1 - b2) / (slope2 - slope1)

    println(s"Part1: $p1")
    println(s"Part2: $p2")
}

def evaluate(monkey: String, expressions: Map[String, Expression]): BigDecimal = {
    expressions.get(monkey).get match {
        case (num: BigDecimal) => num
        case (m1: String, op: String, m2: String) => {
            val num1 = evaluate(m1, expressions)
            val num2 = evaluate(m2, expressions)
            op match {
                case "+" => num1 + num2
                case "-" => num1 - num2
                case "*" => num1 * num2
                case "/" => num1 / num2
            }
        }
    }
}

def line(monkey: String, expressions: Map[String, Expression]): (BigDecimal, BigDecimal) = {
    val (x1, y1) = (0, evaluate(monkey, expressions.updated("humn", 0)))
    val (x2, y2) = (1, evaluate(monkey, expressions.updated("humn", 1)))
    val slope = (y2 - y1) / (x2 - x1)
    val b = evaluate(monkey, expressions.updated("humn", 0))
    (slope, b)
}
