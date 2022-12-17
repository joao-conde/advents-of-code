import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day10"))(_.mkString).get
    val instructions = input.split("\n").map(_.split(" "))

    val xs = instructions.foldLeft(Array((1, 1)))((acc, instruction) => {
        val cpu = acc.last
        val next = instruction match {
            case Array("noop") => Array((cpu(0), cpu(1) + 1))
            case Array("addx", op1) =>
                Array((cpu(0), cpu(1) + 1), (cpu(0) + op1.toInt, cpu(1) + 2))
        }
        acc ++ next
    })

    val signals = List(20, 60, 100, 140, 180, 220)
    val p1 = xs.filter((_, cycle) => signals.contains(cycle)).map(_ * _).sum

    val p2 = xs
        .map((x, cycle) => x)
        .grouped(40)
        .map(r =>
            r.zipWithIndex.map((x, i) => if ((x - 1 to x + 1).contains(i)) "#" else ".").mkString
        )
        .mkString("\n")

    println(s"Part1: $p1")
    println(s"Part2:\n$p2")
}
