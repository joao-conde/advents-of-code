import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day01"))(x => x.mkString).get
    val elfs = input.split("\n\n").map(e => e.split("\n"))
    val sorted = elfs.map(s => s.map(x => x.toInt).sum).sorted.reverse
    println("Part1: " + sorted(0))
    println("Part2: " + sorted.slice(0, 3).sum)
}
