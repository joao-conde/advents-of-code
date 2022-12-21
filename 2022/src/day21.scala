import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.util.Using

type Expression = (String, String, String) | Long

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day21"))(_.mkString).get

    val re1 = """(\w+): (-?\d+)""".r
    val re2 = """(\w+): (\w+) ([+\-*\/]) (\w+)""".r
    val expressions = input
        .split("\n")
        .foldLeft(Map[String, Expression]())((acc, x) => {
            val expression = x match {
                case re1(m1, num)        => m1 -> num.toLong
                case re2(m1, m2, op, m3) => m1 -> (m2, op, m3)
            }
            acc + expression
        })

    val p1 = evaluate("root", expressions)
    println(s"Part1: $p1")
}

def evaluate(
    monkey: String,
    expressions: Map[String, Expression],
    cache: Map[String, Long] = Map()
): Long = {
    if (cache.contains(monkey))
        return cache(monkey)

    val num = expressions.get(monkey).get match {
        case (num: Long) => num
        case (m1: String, op: String, m2: String) => {
            val num1 = evaluate(m1, expressions, cache)
            val num2 = evaluate(m2, expressions, cache)
            op match {
                case "+" => num1 + num2
                case "-" => num1 - num2
                case "*" => num1 * num2
                case "/" => num1 / num2
            }
        }
    }
    cache.addOne(monkey, num)
    num
}
