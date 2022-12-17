import scala.collection.mutable.Set
import scala.io.Source.fromFile
import scala.math.max
import scala.util.Using

type Rock = Set[(Int, Int)]

class Chamber(input: String) {
    val width = 7
    var height = 0
    var solid = (0 until width).map(j => (0, j)).toSet

    var jetIdx = 0
    val jets = input.toList

    var rockIdx = 0
    val rocks: List[Set[(Int, Int)]] = List(
      Set((0, 0), (0, 1), (0, 2), (0, 3)),
      Set((0, 1), (1, 0), (1, 1), (1, 2), (2, 1)),
      Set((0, 0), (0, 1), (0, 2), (1, 2), (2, 2)),
      Set((0, 0), (1, 0), (2, 0), (3, 0)),
      Set((0, 0), (0, 1), (1, 0), (1, 1))
    )

    def topView(): String = {
        val maxHeight = solid.map({ case (i, _) => i }).max
        val maxHeights = (0 until width).map({ case j =>
            solid.filter({ case (_, js) => j == js }).map({ case (i, _) => i }).max
        })
        maxHeights.map(h => h - maxHeight).toArray.mkString(",")
    }

    def simulate(steps: Long): Long = {
        // 7 ints heights, rockidx, jetidx
        var cache: Map[(String, Int, Int), (Long, Long)] = Map()
        var s: Long = 0
        var additional: Long = 0
        while (s < steps) {
            step()

            val config = (topView(), rockIdx, jetIdx)
            if (cache.contains(config)) {
                val (oldstep, oldheight) = cache.get(config).get
                val repeat = (1000000000000L - s) / (s - oldstep)
                s += (s - oldstep) * repeat
                additional += (height - oldheight) * repeat

                cache = Map()
                // reset cache?
            } else {
                cache = cache + (config -> (s, height))
                s += 1
            }
        }
        height + additional
    }

    def step() = {
        var rock = rocks(rockIdx).map({ case (i, j) => (i + height + 4, j + 2) })
        rockIdx = (rockIdx + 1) % rocks.length

        var break = false
        while (!break) {
            rock = jetPush(rock)

            if (canMoveBottom(rock))
                rock = rockMoveDown(rock)
            else
                break = true
        }

        height = max(height, rockHeight(rock))
        rock.foreach(p => solid = solid + p)
    }

    def jetPush(rock: Rock): Rock = {
        val jet = jets(jetIdx)
        jetIdx = (jetIdx + 1) % jets.length

        jet match {
            case '<' if canMoveLeft(rock)  => rockMoveLeft(rock)
            case '>' if canMoveRight(rock) => rockMoveRight(rock)
            case _                         => rock
        }
    }

    def rockHeight(rock: Rock): Int = rock.map({ case (i, _) => i }).max

    def canMoveLeft(rock: Rock): Boolean = {
        rock.map({ case (_, j) => j }).min > 0
        && rock.forall({ case (i, j) => !solid.contains((i, j - 1)) })
    }

    def canMoveRight(rock: Rock): Boolean = {
        rock.map({ case (_, j) => j }).max < width - 1
        && rock.forall({ case (i, j) => !solid.contains((i, j + 1)) })
    }

    def canMoveBottom(rock: Rock): Boolean = {
        rock.forall({ case (i, j) => !solid.contains((i - 1, j)) })
    }

    def rockAtBottom(rock: Rock): Boolean =
        rock.exists({ case (i, j) => solid.contains((i - 1, j)) })

    def rockMoveLeft(rock: Rock): Rock =
        rock.map({ case (i, j) => (i, j - 1) })

    def rockMoveRight(rock: Rock): Rock =
        rock.map({ case (i, j) => (i, j + 1) })

    def rockMoveDown(rock: Rock): Rock =
        rock.map({ case (i, j) => (i - 1, j) })
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day17"))(_.mkString).get

    val p1 = Chamber(input).simulate(2022)
    val p2 = Chamber(input).simulate(1000000000000L) - 1

    println(s"Part1: ${p1}")
    println(s"Part2: ${p2}")
}
