import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day25"))(_.mkString).get

    (1 to 20).foreach(i => println(toSnafu(i)))

    toDecimal("1121-1110-1=0")
}

def toSnafu(x: Int): String = {
    ""
}

def toDecimal(x: String): Int = {
    val digits = Map('2' -> 2, '1' -> 1, '0' -> 0, '-' -> -1, '=' -> -2)
    val factors = (1 to x.length - 1).scanLeft(1)((factor, _) => factor * 5)
    x.map(digits(_)).zip(factors.reverse).map((x, factor) => x * factor).sum
}
