import scala.collection.mutable.{Map, Set}
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
    
    val starts = heightmap.zipWithIndex.flatMap((r, i) => r.zipWithIndex.map((height, j) => (i, j, height)))
            .filter((_, _, height) => height == 'a')

    val p1 = bfs(heightmap, (s0, s1), (e0, e1)).size
    val p2 = starts.map((i, j, _) => bfs(heightmap, (i, j), (e0, e1)).size).min
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def bfs(heightmap: Array[String], src: Node, dst: Node): List[Node] = {
    val visited = Set[Node]()
    var queue = List[(Node, List[Node])]((src, List()))
    while (queue.nonEmpty) {
        val ((i, j), path) = queue.head
        queue = queue.tail

        val height = heightmap(i)(j)

        if ((i, j) == dst)
            return path

        if (!visited.contains((i, j))) {
            if (i > 0 && heightmap(i - 1)(j) - height <= 1) queue = queue :+ ((i-1, j), path :+ (i-1, j))
            if (i < heightmap.length - 1 && heightmap(i + 1)(j) - height <= 1) queue = queue :+ ((i+1, j), path :+ (i+1, j))
            if (j > 0 && heightmap(i)(j - 1) - height <= 1) queue = queue :+ ((i, j-1), path :+ (i, j-1))
            if (j < heightmap(i).length - 1 && heightmap(i)(j + 1) - height <= 1) queue = queue :+ ((i, j+1), path :+ (i, j+1))        
        }

        visited.add((i, j))
    }

    List()
}

def floydWarshall(heightmap: Array[String]): Map[(Node, Node), Double] = {
    val height = heightmap.length
    val width = heightmap(0).length

    var dists = Map[(Node, Node), Double]().withDefaultValue(Double.PositiveInfinity)
    val nodes = (0 to height).flatMap(i => (0 to width).map(j => (i, j)))

    for (i <- 0 until height) {
        for (j <- 0 until width) {
            val cur = (i, j)
            val height = heightmap(i)(j)

            dists += ((cur, cur) -> 0)
    
            val neighbors = List() :::
                (if (i > 0 && heightmap(i - 1)(j) - height <= 1) List((i-1, j)) else List()) :::
                (if (i < heightmap.length - 1 && heightmap(i + 1)(j) - height <= 1) List((i+1, j)) else List()) :::
                (if (j > 0 && heightmap(i)(j - 1) - height <= 1) List((i, j-1)) else List()) :::
                (if (j < heightmap(i).length - 1 && heightmap(i)(j + 1) - height <= 1) List((i, j+1)) else List())
            
            neighbors.foreach(neighbor => dists += ((cur, neighbor) -> 1))
        }
    }

    for (k <- nodes) {
        for (i <- nodes) {
            for (j <- nodes) {
                if (dists((i, j)) > dists((i, k)) + dists((k, j)))
                    dists((i, j)) = dists((i, k)) + dists((k, j))
            }
        }
    }

    return dists
}
