import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day06"))(_.mkString).get
    val p1 = findMarker(input, 4)
    val p2 = findMarker(input, 14)
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def findMarker(buffer: String, length: Int): Int =
    buffer
        .sliding(length)
        .zipWithIndex
        .find((x, i) => x.distinct.length() == length)
        .get(1) + length
