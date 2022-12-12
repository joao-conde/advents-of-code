import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day01"))(_.mkString).get
    val calories = input.split("\n\n").map(_.split("\n").map(_.toInt))
    val top3 = calories.map(_.sum).sorted.reverse.take(3)
    println(s"Part1: ${top3(0)}")
    println(s"Part2: ${top3.sum}")
}
