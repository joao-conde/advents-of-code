import scala.io.Source.fromFile
import scala.util.Using
import scala.collection.mutable.Map

case class File(name: String, size: Int)
case class Dir(name: String, parent: Option[Dir], var contents: Array[File | Dir])

def findSizes(entry: Dir | File, sizes: Map[String, Int]): Int = {
    entry match {
        case File(_, size) => size
        case Dir(name, _, contents) =>
            contents.foldLeft(0)((acc, x) => {
                val size = findSizes(x, sizes)
                sizes(name) += size
                acc + size
            })
    }
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day07"))(_.mkString).get

    val dirs = Map[String, Dir]()
    val commands = input.split("\n");

    val cd = """\$ cd ([\w \/]+)""".r
    val dir = """dir (.+)""".r
    val file = """(\d+) (.+)""".r

    val root = Dir("/", None, Array())
    dirs += "/" -> root

    var cur = root
    commands.foreach(c => {
        c match {
            case "$ cd /"  => cur = root
            case "$ cd .." => cur = cur.parent.getOrElse(root)
            case cd(dir)   => cur = dirs(dir)
            case dir(name) => {
                val child = Dir(name, Some(cur), Array())
                cur.contents = cur.contents :+ child
                dirs += name -> child
            }
            case file(size, name) => {
                val child = File(name, size.toInt)
                cur.contents = cur.contents :+ child
            }
            case _ =>
        }
    })

    val sizes = Map[String, Int]().withDefaultValue(0)
    findSizes(root, sizes)

    val p1 = sizes.values.filter(_ < 100000).sum

    println("Part1: " + p1)
    println("Part2: ")
}
