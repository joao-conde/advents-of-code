import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.math.abs
import scala.util.Using

case class Side(x: Double, y: Double, z: Double)
case class Cube(x: Double, y: Double, z: Double) {
    val sides = List(
      Side(x + 0.5, y, z),
      Side(x - 0.5, y, z),
      Side(x, y + 0.5, z),
      Side(x, y - 0.5, z),
      Side(x, y, z + 0.5),
      Side(x, y, z - 0.5)
    )
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day18"))(_.mkString).get
    val cubes = input.split("\n").map(_.split(",").map(_.toInt)).map(c => Cube(c(0), c(1), c(2)))

    val sides: Map[Side, Int] = Map().withDefaultValue(0)
    cubes.flatMap(_.sides).foreach(side => sides.update(side, sides(side) + 1))

    val p1 = sides.count({ case (_, count) => count == 1 })
    println(s"Part1: $p1")
}
