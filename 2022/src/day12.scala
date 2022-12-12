import scala.collection.mutable.Set
import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day12"))(_.mkString).get

    var heightmap = input.split("\n")

    val heights = heightmap.zipWithIndex
        .flatMap((r, i) => r.zipWithIndex.map((height, j) => (i, j, height)))
    val (s0, s1, _) = heights.find((i, j, h) => h == 'S').get
    val (e0, e1, _) = heights.find((i, j, h) => h == 'E').get
    val starts = heights.filter((_, _, h) => h == 'a')

    heightmap = heightmap.map(r => r.replace("S", "a").replace("E", "z"))

    val p1 = bfs(heightmap, (s0, s1), (e0, e1)).size - 1
    val p2 = starts.map((i, j, _) => bfs(heightmap, (i, j), (e0, e1)).size - 1).filter(_ > 0).min
    println(s"Part1: ${p1}")
    println(s"Part2: ${p2}")
}

def bfs(heightmap: Array[String], src: (Int, Int), dst: (Int, Int)): Array[(Int, Int)] = {
    val visited = Set[(Int, Int)]()
    var queue = List(Array(src))
    while (queue.nonEmpty) {
        val path = queue.head
        queue = queue.tail

        val (i, j) = path.last
        if ((i, j) == dst)
            return path

        if (!visited.contains((i, j)))
            queue = queue ++ neighbors(heightmap, (i, j)).map(n => path :+ n)

        visited.add((i, j))
    }

    Array()
}

def neighbors(heightmap: Array[String], src: (Int, Int)): Array[(Int, Int)] = {
    val (i, j) = src
    val height = heightmap(i)(j)
    var neighbors = Array[(Int, Int)]()
    if (i > 0 && heightmap(i - 1)(j) - height <= 1) neighbors = neighbors :+ (i - 1, j)
    if (i < heightmap.length - 1 && heightmap(i + 1)(j) - height <= 1)
        neighbors = neighbors :+ (i + 1, j)
    if (j > 0 && heightmap(i)(j - 1) - height <= 1) neighbors = neighbors :+ (i, j - 1)
    if (j < heightmap(i).length - 1 && heightmap(i)(j + 1) - height <= 1)
        neighbors = neighbors :+ (i, j + 1)
    neighbors
}
