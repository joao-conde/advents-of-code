import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day06"))(_.mkString).get
    val p1 = findMarker(input, 4)
    val p2 = findMarker(input, 14)
    println(s"Part1: ${p1}")
    println(s"Part2: ${p2}")
}

def findMarker(buffer: String, length: Int): Int =
    buffer.sliding(length).indexWhere(_.distinct.length() == length) + length
