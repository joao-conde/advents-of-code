import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.util.Using

type INode = File | Dir;
case class File(name: String, size: Int)
case class Dir(name: String, parent: Option[Dir], var children: Array[INode])

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day07"))(_.mkString).get

    val root = buildFileSystem(input.split("\n"))
    val sizes = computeSizes(root)

    val p1 = sizes.values.filter(_ < 100000).sum
    val p2 = sizes.values.toArray.sorted.find(x => 70000000 - sizes("/") + x >= 30000000).get
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def buildFileSystem(commands: Array[String]): INode = {
    val cd = """\$ cd ([\w \/]+)""".r
    val dir = """dir (.+)""".r
    val file = """(\d+) (.+)""".r

    val root = Dir("/", None, Array())
    var cwd = root
    commands.foreach(
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
          case dir(name) => {
              val child = Dir(cwd.name + name, Some(cwd), Array())
              cwd.children = cwd.children :+ child
          }
          case file(size, name) => {
              val child = File(name, size.toInt)
              cwd.children = cwd.children :+ child
          }
          case _ =>
      }
    )
    root
}

def computeSizes(node: INode): Map[String, Int] = {
    val sizes = Map[String, Int]().withDefaultValue(0)
    computeSizes(node, sizes)
    sizes
}

def computeSizes(node: INode, sizes: Map[String, Int]): Int = {
    node match {
        case File(_, size) => size
        case Dir(name, _, children) =>
            children.foldLeft(0)((acc, n) => {
                val size = computeSizes(n, sizes)
                sizes(name) += size
                acc + size
            })
    }
}
