import scala.collection.mutable.Set
import scala.io.Source.fromFile
import scala.util.Using

object Day18 {
    case class Cube(x: Int, y: Int, z: Int) {
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
        val cubes = input
            .split("\n")
            .map(_.split(",").map(_.toInt))
            .map(c => Cube(c(0), c(1), c(2)))

        val boxMin = cubes.flatMap(c => List(c.x, c.y, c.z)).min
        val boxMax = cubes.flatMap(c => List(c.x, c.y, c.z)).max

        val p1 = cubes.map(c1 => c1.neighbors().count(c2 => !cubes.contains(c2))).sum
        val p2 =
            cubes.map(c1 => c1.neighbors().count(c2 => reachesOut(c2, cubes, boxMin, boxMax))).sum

        println(s"Part1: $p1")
        println(s"Part2: $p2")
    }

    def reachesOut(cube: Cube, cubes: Array[Cube], boxMin: Int, boxMax: Int): Boolean = {
        val seen: Set[Cube] = Set()
        var queue = List(cube)
        while (queue.nonEmpty) {
            val cur = queue.head
            queue = queue.tail

            val skip = seen.contains(cur) || cubes.contains(cur)
            seen.add(cur)

            if (!skip) {
                val outside = cur.x < boxMin || cur.x > boxMax ||
                    cur.y < boxMin || cur.y > boxMax ||
                    cur.z < boxMin || cur.z > boxMax
                if (outside) return true

                queue = queue ::: cur.neighbors()
            }
        }

        false
    }
}
