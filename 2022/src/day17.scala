import scala.collection.mutable.{Map, Set}
import scala.io.Source.fromFile
import scala.math.max
import scala.util.Using

type Rock = Set[(Int, Int)]
case class Config(heights: List[Int], rockIdx: Int, jetIdx: Int)
case class State(step: Long, height: Int)

class Chamber(input: String) {
    val WIDTH = 7

    var height = 0
    var terrain = Set((0 until WIDTH).map(j => (0, j)): _*)

    var jetIdx = 0
    val jets = input

    var rockIdx = 0
    val rocks = Array(
      Set((0, 0), (0, 1), (0, 2), (0, 3)),
      Set((0, 1), (1, 0), (1, 1), (1, 2), (2, 1)),
      Set((0, 0), (0, 1), (0, 2), (1, 2), (2, 2)),
      Set((0, 0), (1, 0), (2, 0), (3, 0)),
      Set((0, 0), (0, 1), (1, 0), (1, 1))
    )

    def simulate(steps: Long): Long = {
        // caches previous chamber configurations knowing
        // if the same configuration appears, the height
        // is going to increment in the same way next
        // as it did from then until now
        var cache: Map[Config, State] = Map()

        // extra height added counter because directly changing
        // the height of the chamber would yield incorrect
        // computation of chamber configs
        var extra = 0L

        var curStep = 0L
        while (curStep < steps) {
            // one simulation step (drop one rock)
            step()

            val config = getConfig()
            cache.get(config) match {
                case Some(State(prevStep, prevHeight)) => {
                    // compute how many times to repeat i.e.
                    // how many steps to fast-forward
                    val repeat = (steps - curStep) / (curStep - prevStep)
                    extra += (height - prevHeight) * repeat
                    curStep += (curStep - prevStep) * repeat
                    cache.clear()
                }
                case _ => {
                    val state = State(curStep, height)
                    cache.addOne(config -> state)
                    curStep += 1
                }
            }
        }

        // returns current chamber height plus the
        // height added with fast-forwards
        height + extra
    }

    def getConfig(): Config = Config(normalizedHeights(), rockIdx, jetIdx)

    def normalizedHeights(): List[Int] = {
        // finds out the max height of each column
        // and normalizes them by subtracting the tallest one
        val maxHeights = (0 until WIDTH).map({ case j =>
            terrain.filter({ case (_, tj) => j == tj }).map({ case (i, _) => i }).max
        })
        val maxHeight = maxHeights.max
        maxHeights.map(h => h - maxHeight).toList
    }

    def step() = {
        // build a new rock with current shape, offset by
        // 4 units high and 2 to the right
        var rock = nextRock()

        // while the rock is not settled, the jet pushes
        // it and the rock moves down, if possible
        var settled = false
        while (!settled) {
            rock = jetPush(rock)

            if (canMoveBottom(rock))
                rock = moveDown(rock)
            else
                settled = true
        }

        // update chamber max height tracker and add new
        // settled rock as terrain
        height = max(height, rockHeight(rock))
        terrain.addAll(rock)
    }

    def nextRock(): Rock = {
        var rock = rocks(rockIdx).map({ case (i, j) => (i + height + 4, j + 2) })
        rockIdx = (rockIdx + 1) % rocks.length
        rock
    }

    def nextJet(): Char = {
        val jet = jets(jetIdx)
        jetIdx = (jetIdx + 1) % jets.length
        jet
    }

    def jetPush(rock: Rock): Rock = {
        val jet = nextJet()
        jet match {
            case '<' if canMoveLeft(rock)  => moveLeft(rock)
            case '>' if canMoveRight(rock) => moveRight(rock)
            case _                         => rock
        }
    }

    def rockHeight(rock: Rock): Int = rock.map({ case (i, _) => i }).max

    def canMoveLeft(rock: Rock): Boolean = rock.map({ case (_, j) => j }).min > 0 && rock.forall({
        case (i, j) => !terrain.contains((i, j - 1))
    })

    def canMoveRight(rock: Rock): Boolean =
        rock.map({ case (_, j) => j }).max < WIDTH - 1 && rock.forall({ case (i, j) =>
            !terrain.contains((i, j + 1))
        })

    def canMoveBottom(rock: Rock): Boolean = rock.forall({ case (i, j) =>
        !terrain.contains((i - 1, j))
    })

    def moveLeft(rock: Rock): Rock = rock.map({ case (i, j) => (i, j - 1) })

    def moveRight(rock: Rock): Rock = rock.map({ case (i, j) => (i, j + 1) })

    def moveDown(rock: Rock): Rock = rock.map({ case (i, j) => (i - 1, j) })
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day17"))(_.mkString).get
    val p1 = Chamber(input).simulate(2022)
    val p2 = Chamber(input).simulate(1000000000000L) - 1
    println(s"Part1: $p1")
    println(s"Part2: $p2")
}
