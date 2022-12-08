import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.util.Using

type INode = File | Dir;
case class File(name: String, size: Int)
case class Dir(name: String, parent: Option[Dir] = None, var children: List[INode] = List())

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day07"))(_.mkString).get
    val root = buildFileSystem(input.split("\n"))
    val sizes = computeSizes(root)
    val p1 = sizes.values.filter(_ < 100000).sum
    val p2 = sizes.values.toArray.sorted.find(70000000 - sizes("/") + _ >= 30000000).get
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def buildFileSystem(lines: Array[String]): INode = {
    val cd = """\$ cd ([\w \/ ..]+)""".r
    val dir = """dir (.+)""".r
    val file = """(\d+) (.+)""".r

    val root = Dir("/")
    var cwd = root
    lines.foreach({
        case cd(dir)          => cwd = changeDirectory(root, cwd, dir)
        case dir(name)        => cwd.children = cwd.children :+ Dir(cwd.name + name, Some(cwd))
        case file(size, name) => cwd.children = cwd.children :+ File(name, size.toInt)
        case _                =>
    })

    root
}

def changeDirectory(root: Dir, cwd: Dir, arg: String): Dir = {
    arg match {
        case "/"  => root
        case ".." => cwd.parent.getOrElse(root)
        case dir =>
            cwd.children
                .collectFirst({ case subdir: Dir if subdir.name == cwd.name + dir => subdir })
                .get
    }
}

def computeSizes(root: INode): Map[String, Int] = {
    def compute(root: INode, sizes: Map[String, Int]): Int = {
        root match {
            case File(_, size) => size
            case Dir(name, _, children) =>
                children.foldLeft(0)((acc, node) => {
                    val size = compute(node, sizes)
                    sizes(name) += size
                    acc + size
                })
        }
    }

    val sizes = Map[String, Int]().withDefaultValue(0)
    compute(root, sizes)
    sizes
}
