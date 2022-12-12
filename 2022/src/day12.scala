import scala.collection.mutable.Set
import scala.io.Source.fromFile
import scala.util.Using

type Node = (Int, Int)

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day12"))(_.mkString).get

    var heightmap = input.split("\n")
    val s0 = heightmap.indexWhere(_.contains('S'))
    val s1 = heightmap(s0).indexWhere(_ == 'S')
    val e0 = heightmap.indexWhere(_.contains('E'))
    val e1 = heightmap(e0).indexWhere(_ == 'E')
    heightmap = heightmap.map(r => r.replace("S", "a").replace("E", "z"))
    
    val starts = heightmap
                    .zipWithIndex
                    .flatMap((r, i) => r.zipWithIndex.map((height, j) => (i, j, height)))
                    .filter((_, _, height) => height == 'a')

    val p1 = bfs(heightmap, (s0, s1), (e0, e1)).size - 1
    val p2 = starts.map((i, j, _) => bfs(heightmap, (i, j), (e0, e1)).size - 1).filter(_ > 0).min
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def bfs(heightmap: Array[String], src: Node, dst: Node): Array[Node] = {
    val visited = Set[Node]()
    var queue = List(Array(src))
    while (queue.nonEmpty) {
        val path = queue.head
        queue = queue.tail

        val (i, j) = path.last

        val height = heightmap(i)(j)

        if ((i, j) == dst)
            return path

        if (!visited.contains((i, j))) {
            if (i > 0 && heightmap(i - 1)(j) - height <= 1) queue = queue :+ (path :+ (i-1, j))
            if (i < heightmap.length - 1 && heightmap(i + 1)(j) - height <= 1) queue = queue :+ (path :+ (i+1, j))
            if (j > 0 && heightmap(i)(j - 1) - height <= 1) queue = queue :+ (path :+ (i, j-1))
            if (j < heightmap(i).length - 1 && heightmap(i)(j + 1) - height <= 1) queue = queue :+ (path :+ (i, j+1))      
        }

        visited.add((i, j))
    }

    Array()
}
