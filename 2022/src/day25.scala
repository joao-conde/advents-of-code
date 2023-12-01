import scala.io.Source.fromFile
import scala.util.Using

object Day25 {
    def main(args: Array[String]): Unit = {
        val input = Using(fromFile("input/day25"))(_.mkString).get
        val p1 = toSnafu(input.split("\n").map(toDecimal).sum)
        println(s"Part1: $p1")
    }

    def toSnafu(base10: Long): String =
        if (base10 == 0) return ""
        base10 % 5 match {
            case 4 => toSnafu(base10 / 5 + 1) + "-"
            case 3 => toSnafu(base10 / 5 + 1) + "="
            case d => toSnafu(base10 / 5) + d
        }

    def toDecimal(snafu: String): Long =
        if (snafu.isEmpty) return 0L
        snafu.last match {
            case '-' => 5 * toDecimal(snafu.init) - 1
            case '=' => 5 * toDecimal(snafu.init) - 2
            case d   => 5 * toDecimal(snafu.init) + d.asDigit
        }
}
