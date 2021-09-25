console.log("🎄 Advent of Code 2018\n");
for (let d = 1; d <= 25; d += 1) {
    console.log(`> Day ${d}`);
    const day = d >= 10 ? d : `0${d}`;
    console.log(`node src/${day}.js`);
    console.log();
}
