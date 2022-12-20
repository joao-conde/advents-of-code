import scala.io.Source.fromFile
import scala.util.Using

class Num(val value: Long, var prev: Option[Num], var next: Option[Num])

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day20"))(_.mkString).get
    val nums = input.split("\n").map(_.toInt)

    val p1 = decrypt(nums)
    val p2 = decrypt(nums, rounds = 10, key = 811589153L)
    println(s"Part1: $p1")
    println(s"Part2: $p2")
}

def decrypt(nums: Array[Int], rounds: Int = 1, key: Long = 1): Long = {
    val nodes = nums.map(i => Num(i * key, None, None))
    nodes
        .zip(nodes.drop(1))
        .foreach((n1, n2) => {
            n1.next = Some(n2)
            n2.prev = Some(n1)
        })
    nodes.last.next = Some(nodes.head)
    nodes.head.prev = Some(nodes.last)

    (1 to rounds).foreach(_ => mix(nodes))

    val zero = nodes.find(_.value == 0).get
    List(1000, 2000, 3000)
        .map(i => {
            var cur = zero
            (1 to i).foreach(_ => cur = cur.next.get)
            cur.value
        })
        .sum
}

def mix(nodes: Array[Num]) = {
    nodes.foreach(n => {
        n.prev.get.next = n.next
        n.next.get.prev = n.prev

        val move = (n.value % (nodes.length - 1)).toInt

        var (left, right) = (n.prev.get, n.next.get)
        if (move >= 0)
            (1 to move).foreach(_ => {
                left = left.next.get
                right = right.next.get
            })
        else
            (1 to move.abs).foreach(_ => {
                left = left.prev.get
                right = right.prev.get
            })

        n.prev = Some(left)
        n.next = Some(right)
        left.next = Some(n)
        right.prev = Some(n)
    })
}
