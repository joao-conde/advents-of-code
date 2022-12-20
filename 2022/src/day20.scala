import scala.io.Source.fromFile
import scala.math.abs
import scala.util.Using

class Num(val value: Int, var prev: Option[Num], var next: Option[Num])

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day20"))(_.mkString).get

    val nums = input.split("\n").map(i => Num(i.toInt, None, None))
    nums.zip(nums.drop(1))
        .foreach((n1, n2) => {
            n1.next = Some(n2)
            n2.prev = Some(n1)
        })
    nums.last.next = Some(nums.head)
    nums.head.prev = Some(nums.last)

    nums.foreach(n => {
        n.prev.get.next = n.next
        n.next.get.prev = n.prev

        val move = n.value

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

    val zero = nums.find(_.value == 0).get
    val p1 = List(1000, 2000, 3000)
        .map(i => {
            var cur = zero
            (1 to i).foreach(_ => cur = cur.next.get)
            cur.value
        })
        .sum

    println(p1)
}
