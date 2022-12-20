import scala.io.Source.fromFile
import scala.math.abs
import scala.util.Using

// TODO swap other days ::: to ++

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day20"))(_.mkString).get

    val nums = input.split("\n").map(_.toInt)
    var indexes = (0 until nums.length).toList
    (0 until nums.length).foreach(i => {
        val num = nums(i)
        val at = indexes.indexWhere(_ == i)
        val dst = wrap(at + num, 0, nums.length - 1)
        val (front, back) = indexes.splitAt(dst + 1)
        indexes = front.filter(_ != i) ++ List(i) ++ back.filter(_ != i)
    })

    val zero = indexes.indexWhere(nums(_) == 0)
    val p1 = List(1000, 2000, 3000).map(i => nums(indexes(wrap(i + zero, 0, nums.length - 1)))).sum

    println(s"Part1: $p1")
    println(s"Part2: ${}")
}

def wrap(i: Int, from: Int, to: Int): Int = (if (i <= from) to - abs(from - i) else i) % (to + 1)
