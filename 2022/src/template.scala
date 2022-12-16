import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/dayXX"))(_.mkString).get
    println(s"Part1: ${}")
    println(s"Part2: ${}")
}
