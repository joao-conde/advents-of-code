// import scala.collection.mutable.{}
import scala.io.Source.fromFile
import scala.math.abs
import scala.util.Using

case class Cube(x: Double, y: Double, z: Double)
case class Side(x: Double, y: Double, z: Double)

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day18"))(_.mkString).get

    val cubes = input.split("\n").map(_.split(",").map(_.toInt)).map(c => Cube(c(0), c(1), c(2)))

    val sides = cubes
        .flatMap(adjacent)
        .foldLeft(Map[Side, Int]().withDefaultValue(0))((sides, side) =>
            sides.updated(side, sides(side) + 1)
        )
    val p1 = sides.count({ case (_, count) => count == 1 })

    println(s"Part1: $p1")
}

def adjacent(cube: Cube): List[Side] =
    List(
      Side(cube.x + 0.5, cube.y, cube.z),
      Side(cube.x - 0.5, cube.y, cube.z),
      Side(cube.x, cube.y + 0.5, cube.z),
      Side(cube.x, cube.y - 0.5, cube.z),
      Side(cube.x, cube.y, cube.z + 0.5),
      Side(cube.x, cube.y, cube.z - 0.5)
    )
