import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day10"))(_.mkString).get
    val instructions = input.split("\n").map(_.split(" "))

    val xs = instructions.foldLeft(Array((1, 1)))((acc, instruction) => {
        val cpu = (acc.last(0), acc.last(1))
        acc :+
            (instruction match {
                case Array("noop")      => (cpu(0), cpu(1) + 1)
                case Array("addx", op1) => (cpu(0) + op1.toInt, cpu(1) + 2)
            })
    })

    val p1 =
        xs.filter((x, cycles) => List(20, 60, 100, 140, 180, 220).contains(cycles)).map(_ * _).sum

    println("Part1: " + p1)
    println("Part2: ")
}
