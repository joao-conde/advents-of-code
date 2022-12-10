import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day10"))(_.mkString).get
    val instructions = input.split("\n").map(_.split(" "))

    val xs = instructions.foldLeft(Array((1, 1)))((acc, instruction) => {
        val cpu = (acc.last(0), acc.last(1))
        acc ++
            (instruction match {
                case Array("noop") => Array((cpu(0), cpu(1) + 1))
                case Array("addx", op1) =>
                    Array((cpu(0), cpu(1) + 1), (cpu(0) + op1.toInt, cpu(1) + 2))
            })
    })

    val signals = List(20, 60, 100, 140, 180, 220)
    val p1 = xs.filter((x, cycle) => signals.contains(cycle)).map(_ * _).sum

    println("Part1: " + p1)
    println("Part2: (see below)")
    xs.grouped(40)
        .foreach(l => {
            l.zipWithIndex.foreach({
                case ((x, cycle), i) => {
                    val char = if ((x - 1 to x + 1).contains(i)) "#" else "."
                    print(char)
                }
            })
            println()
        })
}
