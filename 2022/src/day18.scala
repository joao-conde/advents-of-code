import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.math.abs
import scala.util.Using

case class Cube(x: Double, y: Double, z: Double) {
    def neighbors(): List[Cube] = List(
      Cube(x + 1, y, z),
      Cube(x - 1, y, z),
      Cube(x, y + 1, z),
      Cube(x, y - 1, z),
      Cube(x, y, z + 1),
      Cube(x, y, z - 1)
    )
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day18"))(_.mkString).get
    val cubes =
        input.split("\n").map(_.split(",").map(_.toInt)).map(c => Cube(c(0), c(1), c(2))).toSet
    val p1 = cubes.toList.map(cube => cube.neighbors().count(c => !cubes.contains(c))).sum
    println(p1)
}
