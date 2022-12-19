import scala.collection.mutable.Map
import scala.io.Source.fromFile
import scala.util.Using

enum Resource:
    case Ore, Clay, Obsidian

enum Robot:
    case Ore, Clay, Obsidian, Geode

type Costs = Map[Robot, List[(Resource, Int)]]
type State = (Map[Robot, Int], Map[Resource, Int], Int)

class Blueprint(id: Int, costs: Costs) {

    def strategies() = {
        var states: List[List[State]] = List()
        var queue: List[State] = List(
          (
            Map(Robot.Ore -> 1).withDefaultValue(0),
            Map().withDefaultValue(0),
            1
          )
        )
        while (queue.nonEmpty) {
            var curState = queue.head
            var (robots, resources, minutes) = curState
            queue = queue.tail

            val possibilities = Robot.values
                .filter(robot => canBuild(robot, resources))

            possibilities.foreach(robot => build(robot, resources, robots))

            if (minutes == 24) {
                val state = (robots, resources, minutes + 1)
                queue = queue :+ state
            } else {
                states = states ::: curState
            }
        }
        states
    }

    def canBuild(robot: Robot, available: Map[Resource, Int]): Boolean =
        costs(robot).forall((resource, quantity) => available(resource) >= quantity)

    def build(robot: Robot, available: Map[Resource, Int], robots: Map[Robot, Int]) = {
        costs(robot).foreach((resource, quantity) =>
            available.update(resource, available(resource) - quantity)
        )
        robots.update(robot, robots(robot) + 1)
    }
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day19"))(_.mkString).get

    val bp1 = Blueprint(
      1,
      Map(
        Robot.Ore -> List((Resource.Ore, 4)),
        Robot.Clay -> List((Resource.Ore, 2)),
        Robot.Obsidian -> List((Resource.Ore, 3), (Resource.Clay, 14)),
        Robot.Geode -> List((Resource.Ore, 2), (Resource.Obsidian, 7))
      )
    )

    val bp2 = Blueprint(
      1,
      Map(
        Robot.Ore -> List((Resource.Ore, 2)),
        Robot.Clay -> List((Resource.Ore, 3)),
        Robot.Obsidian -> List((Resource.Ore, 3), (Resource.Clay, 8)),
        Robot.Geode -> List((Resource.Ore, 3), (Resource.Obsidian, 12))
      )
    )

    println(s"Part1: ${}")
    println(s"Part2: ${}")
}
