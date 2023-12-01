import scala.io.Source.fromFile
import scala.util.Using

object Day20 {
    class Num(val value: Long, var prev: Option[Num], var next: Option[Num]) {
        def append(other: Num) = {
            this.next = Some(other)
            other.prev = Some(this)
        }

        def delete() = {
            this.prev.get.next = this.next
            this.next.get.prev = this.prev
        }

        def lookup(offset: Int) =
            (1 to offset.abs).foldLeft(this)((cur, _) =>
                if (offset > 0) cur.next.get else cur.prev.get
            )
    }

    def main(args: Array[String]): Unit = {
        val input = Using(fromFile("input/day20"))(_.mkString).get
        val nums = input.split("\n").map(_.toInt)

        val p1 = decrypt(nums)
        val p2 = decrypt(nums, rounds = 10, key = 811589153L)
        println(s"Part1: $p1")
        println(s"Part2: $p2")
    }

    def decrypt(nums: Array[Int], rounds: Int = 1, key: Long = 1): Long = {
        val nodes = buildNodes(nums, key)
        (1 to rounds).foreach(_ => mix(nodes))

        val zero = nodes.find(_.value == 0).get
        List(1000, 2000, 3000).map(zero.lookup(_).value).sum
    }

    def mix(nodes: Array[Num]) = {
        nodes.foreach(n => {
            n.delete()
            val move = (n.value % (nodes.length - 1)).toInt
            var left = n.prev.get.lookup(move)
            var right = n.next.get.lookup(move)
            left.append(n)
            n.append(right)
        })
    }

    def buildNodes(nums: Array[Int], key: Long): Array[Num] = {
        val nodes = nums.map(i => Num(i * key, None, None))
        nodes.zip(nodes.drop(1)).foreach((n1, n2) => n1.append(n2))
        nodes.last.append(nodes.head)
        nodes
    }
}
