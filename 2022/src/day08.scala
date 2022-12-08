import scala.io.Source.fromFile
import scala.math.{max, min}
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day08"))(_.mkString).get
    val heights = input.split("\n").map(_.split("").map(_.toInt))
    val dp = input.split("\n").map(_.split("").map(_.toInt))

    var highestScore = -1;
    var visible = 2 * heights.length + 2 * heights(0).length - 4
    for (i <- 1 until heights.length - 1) {
        for (j <- 1 until heights(0).length - 1) {
            val height = heighest(heights, i, j)
            visible = if (heights(i)(j) > height) visible + 1 else visible

            val score = scenicScore(heights, i, j)
            highestScore = max(score, highestScore)
        }
    }

    println("Part1: " + visible)
    println("Part2: " + highestScore)
}

def heighest(heights: Array[Array[Int]], i: Int, j: Int): Int = {
    val up = (0 until i).map(heights(_)(j)).reduce(max)
    val down = (i + 1 until heights.length).map(heights(_)(j)).reduce(max)
    val left = (0 until j).map(heights(i)(_)).reduce(max)
    val right = (j + 1 until heights(i).length).map(heights(i)(_)).reduce(max)
    List(up, down, left, right).reduce(min)
}

def scenicScore(heights: Array[Array[Int]], i: Int, j: Int): Int = {
    val height = heights(i)(j)

    val upRange = (i - 1 to 0 by -1)
    var up = upRange.map(heights(_)(j)).indexWhere(_ >= height)
    up = if (up == -1) upRange.length else up + 1

    val downRange = (i + 1 until heights.length)
    var down = downRange.map(heights(_)(j)).indexWhere(_ >= height)
    down = if (down == -1) downRange.length else down + 1

    val leftRange = (j - 1 to 0 by -1)
    var left = leftRange.map(heights(i)(_)).indexWhere(_ >= height)
    left = if (left == -1) leftRange.length else left + 1

    val rightRange = (j + 1 until heights(i).length)
    var right = rightRange.map(heights(i)(_)).indexWhere(_ >= height)
    right = if (right == -1) rightRange.length else right + 1

    up * down * left * right
}
