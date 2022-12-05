import scala.collection.mutable.Stack
import scala.io.Source.fromFile
import scala.util.Using

def main(args: Array[String]): Unit = {
    val input = Using(fromFile("input/day05"))(_.mkString).get
    var Array(crates, moves) = input.split("\n\n").map(_.split("\n"))
    val p1 = rearrange(buildStacks(crates), moves).map(_.top).mkString
    val p2 = rearrange(buildStacks(crates), moves, reverse = true).map(_.top).mkString
    println("Part1: " + p1)
    println("Part2: " + p2)
}

def buildStacks(crates: Array[String]): Array[Stack[Char]] = {
    crates.toList.transpose
        .map(_.filter(_.isUpper))
        .filter(_.nonEmpty)
        .map(l => Stack().pushAll(l.reverse))
        .toArray
}

def rearrange(
    stacks: Array[Stack[Char]],
    moves: Array[String],
    reverse: Boolean = false
): Array[Stack[Char]] = {
    val re = """move (\d+) from (\d+) to (\d+)""".r
    moves
        .map({ case re(amount, from, to) => (amount.toInt, from.toInt - 1, to.toInt - 1) })
        .foreach((amount, from, to) => move(stacks(from), stacks(to), amount, reverse))
    stacks
}

def move[T](from: Stack[T], to: Stack[T], amount: Int, reverse: Boolean = false): Unit = {
    var popped = (1 to amount).map(i => from.pop())
    if (reverse) popped = popped.reverse
    popped.foreach(x => to.push(x))
}
