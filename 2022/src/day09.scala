import scala.io.Source.fromFile
import scala.math.abs
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day09"))(_.mkString).get
    val moves = input.split("\n").map(_.split(" ")).map({ case Array(x, y) => (x, y.toInt) })

    val headPath = moves
        .flatMap((move, steps) => (1 to steps).map(_ => move))
        .foldLeft(Array((0, 0)))((acc, move) => {
            val head = acc.last
            move match {
                case "R" => acc :+ (head(0) + 1, head(1))
                case "L" => acc :+ (head(0) - 1, head(1))
                case "U" => acc :+ (head(0), head(1) + 1)
                case "D" => acc :+ (head(0), head(1) - 1)
            }
        })

    val p1 = tailPath(2, headPath).toSet.size
    val p2 = tailPath(10, headPath).toSet.size
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def tailPath(length: Int, headPath: Array[(Int, Int)]): Array[(Int, Int)] =
    (1 until length).foldLeft(headPath)((acc, _) => follow(acc))

def follow(path: Array[(Int, Int)]): Array[(Int, Int)] = {
    path.foldLeft(Array((0, 0)))((acc, head) => {
        val tail = acc.last
        val delta = (head(0) - tail(0), head(1) - tail(1))
        val deltaN = (
          if (delta(0) != 0) delta(0) / abs(delta(0)) else 0,
          if (delta(1) != 0) delta(1) / abs(delta(1)) else 0
        )
        val update = abs(delta(0)) > 1 || abs(delta(1)) > 1
        if (update) acc :+ (tail(0) + deltaN(0), tail(1) + deltaN(1)) else acc :+ tail
    })
}
