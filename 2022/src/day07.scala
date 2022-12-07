import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.util.Using

type INode = File | Dir;
case class File(name: String, size: Int)
case class Dir(name: String, parent: Option[Dir], var children: List[INode])

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
    val cd = """\$ cd ([\w \/]+)""".r
    val dir = """dir (.+)""".r
    val file = """(\d+) (.+)""".r

    val root = Dir("/", None, List())

    var cwd = root
    lines.foreach(
      {
          case "$ cd /"  => cwd = root
          case "$ cd .." => cwd = cwd.parent.getOrElse(root)
          case cd(dir) =>
              cwd = cwd.children
                  .find({
                      case (Dir(name, _, _)) => name == cwd.name + dir
                      case _                 => false
                  })
                  .get
                  .asInstanceOf[Dir]
          case dir(name) => cwd.children = cwd.children :+ Dir(cwd.name + name, Some(cwd), List())
          case file(size, name) => cwd.children = cwd.children :+ File(name, size.toInt)
          case _                =>
      }
    )

    root
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
