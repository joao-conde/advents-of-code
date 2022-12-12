import scala.io.Source.fromFile
import scala.math.{max, min}
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day08"))(_.mkString).get
    val heights = input.split("\n").map(_.split("").map(_.toInt))

    val (nrows, ncols) = (heights.length, heights(0).length)
    val coords = (1 until nrows - 1).flatMap(i => (1 until ncols - 1).map(j => (i, j)))

    val p1 = coords.count((i, j) =>
        heights(i)(j) > lowestToEdge(heights, i, j)
    ) + 2 * nrows + 2 * ncols - 4
    val p2 = coords.map((i, j) => scenicScore(heights, i, j)).reduce(max)
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def lowestToEdge(heights: Array[Array[Int]], i: Int, j: Int): Int = {
    val up = (0 until i).map(heights(_)(j)).reduce(max)
    val down = (i + 1 until heights.length).map(heights(_)(j)).reduce(max)
    val left = (0 until j).map(heights(i)(_)).reduce(max)
    val right = (j + 1 until heights(i).length).map(heights(i)(_)).reduce(max)
    List(up, down, left, right).reduce(min)
}

def scenicScore(heights: Array[Array[Int]], i: Int, j: Int): Int = {
    val height = heights(i)(j)
    val up = countTrees((i - 1 to 0 by -1).map(heights(_)(j)), height)
    val down = countTrees((i + 1 until heights.length).map(heights(_)(j)), height)
    val left = countTrees((j - 1 to 0 by -1).map(heights(i)(_)), height)
    val right = countTrees((j + 1 until heights(i).length).map(heights(i)(_)), height)
    up * down * left * right
}

def countTrees(seq: Seq[Int], height: Int): Int = {
    val index = seq.indexWhere(_ >= height)
    if (index == -1) seq.length else index + 1
}
