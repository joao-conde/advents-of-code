import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val lines = Using(fromFile("input/day01"))(x => x.getLines().toList).get
    val ints = lines.map(x => x.toInt)
    val even = ints.filter(x => x % 2 == 0)
    println("Part1: " + even.length)
}
