import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day09"))(_.mkString).get
    val moves = input.split("\n").map(_.split(" ")).map({ case Array(x, y) => (x, y.toInt) })

    val headPath = moves
        .flatMap((move, steps) => (1 to steps).map(_ => move))
        .scanLeft((0, 0))((head, move) => {
            move match {
                case "R" => (head(0) + 1, head(1))
                case "L" => (head(0) - 1, head(1))
                case "U" => (head(0), head(1) + 1)
                case "D" => (head(0), head(1) - 1)
            }
        })

    val p1 = tailPath(2, headPath).toSet.size
    val p2 = tailPath(10, headPath).toSet.size
    println(s"Part1: ${p1}")
    println(s"Part2: ${p2}")
}

def tailPath(length: Int, headPath: Array[(Int, Int)]): Array[(Int, Int)] =
    (1 until length).foldLeft(headPath)((path, _) => follow(path))

def follow(path: Array[(Int, Int)]): Array[(Int, Int)] = {
    path.scanLeft((0, 0))((cur, dst) => {
        val delta = (dst(0) - cur(0), dst(1) - cur(1))
        val update = delta(0).abs > 1 || delta(1).abs > 1
        if (update) (cur(0) + delta(0).signum, cur(1) + delta(1).signum) else cur
    })
}
