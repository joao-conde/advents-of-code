import scala.io.Source.fromFile
import scala.util.Using

class MonkeyInTheMiddle(state: String) {

    val re = """\s*Monkey .:
\s*Starting\ items:\ (.+)
\s*Operation: new = old (.) (.+)
\s*Test: divisible by (\d+)
\s*If true: throw to monkey (\d+)
\s*If false: throw to monkey (\d+)""".r

    val blocks = state.split("\n\n")

    val lcm = blocks.map({ case re(_, _, _, prime, _, _) => prime.toInt }).reduce(_ * _)

    val monkeys = blocks.map({
        case re(items, operator, operand, prime, yes, no) => {
            val itemList = items.split(",").map(s => BigInt(s.trim)).toList
            val op = (operator, operand) match {
                case ("*", "old") => (old: BigInt) => old * old
                case ("*", _)     => (old: BigInt) => old * BigInt(operand)
                case ("+", _)     => (old: BigInt) => old + BigInt(operand)
            }
            val test = (worry: BigInt) => if (worry % prime.toInt == 0) yes.toInt else no.toInt
            Monkey(op, test, itemList)
        }
    })

    def play(rounds: Int, soother: BigInt => BigInt = (old: BigInt) => old % lcm): BigInt = {
        (1 to rounds).foreach(r => monkeys.foreach(m => m.processItems(monkeys, soother)))
        monkeys.map(_.inspected).sorted.takeRight(2).product
    }
}

class Monkey(
    op: BigInt => BigInt,
    test: BigInt => Int,
    var items: List[BigInt],
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
    val p2 = MonkeyInTheMiddle(input).play(10000)
    println(s"Part1: ${p1}")
    println(s"Part2: ${p2}")
}
