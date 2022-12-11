import scala.io.Source.fromFile
import scala.util.Using

class MonkeyInTheMiddle(state: String) {
    val monkeys = Array(
      Monkey(List(83, 62, 93), old => old * 17, worry => if (worry % 2 == 0) 1 else 6),
      Monkey(List(90, 55), old => old + 1, worry => if (worry % 17 == 0) 6 else 3),
      Monkey(List(91, 78, 80, 97, 79, 88), old => old + 3, worry => if (worry % 19 == 0) 7 else 5),
      Monkey(List(64, 80, 83, 89, 59), old => old + 5, worry => if (worry % 3 == 0) 7 else 2),
      Monkey(List(98, 92, 99, 51), old => old * old, worry => if (worry % 5 == 0) 0 else 1),
      Monkey(
        List(68, 57, 95, 85, 98, 75, 98, 75),
        old => old + 2,
        worry => if (worry % 13 == 0) 4 else 0
      ),
      Monkey(List(74), old => old + 4, worry => if (worry % 7 == 0) 3 else 2),
      Monkey(
        List(68, 64, 60, 68, 87, 80, 82),
        old => old * 19,
        worry => if (worry % 11 == 0) 4 else 5
      )
    )

    def play(rounds: Int, soother: BigInt => BigInt): BigInt = {
        (1 to rounds).foreach(r => monkeys.foreach(m => m.processItems(monkeys, soother)))
        monkeys.map(_.inspected).sorted.takeRight(2).product
    }
}

class Monkey(
    var items: List[BigInt],
    op: BigInt => BigInt,
    test: BigInt => Int,
    var inspected: BigInt = 0
) {
    def processItems(peers: Array[Monkey], soother: BigInt => BigInt): Unit = {
        items.foreach(_ => processItem(peers, soother))
    }

    def processItem(peers: Array[Monkey], soother: BigInt => BigInt): Unit = {
        val worry = soother(op(items.head))
        val dst = test(worry)
        peers(dst).items = peers(dst).items :+ worry
        items = items.tail
        inspected += 1
    }
}

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day11"))(_.mkString).get
    val p1 = MonkeyInTheMiddle(input).play(20, worry => worry / 3)
    val p2 = MonkeyInTheMiddle(input).play(10000, worry => worry % 9699690)
    println("Part1: " + p1)
    println("Part2: " + p2)
}
